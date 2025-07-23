use blitz_html::HtmlDocument;
use blitz_net::Provider;
use blitz_dom::net::Resource;
use blitz_traits::net::{NetCallback, NetProvider, BoxedHandler, Request};
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

#[derive(Clone)]
struct PendingCount {
    count: Arc<std::sync::atomic::AtomicUsize>,
}

impl PendingCount {
    fn new() -> Self {
        Self {
            count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    fn current(&self) -> usize {
        self.count.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn increment(&self) {
        self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    fn decrement(&self) {
        self.count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }

    fn is_empty(&self) -> bool {
        self.current() == 0
    }
}

pub struct ErrorHandlingCallback<T>(UnboundedSender<(usize, Result<T, Option<String>>)>);
impl<T> ErrorHandlingCallback<T> {
    pub fn new() -> (UnboundedReceiver<(usize, Result<T, Option<String>>)>, Self) {
        let (send, recv) = unbounded_channel();
        (recv, Self(send))
    }
}
impl<T: Send + Sync + 'static> NetCallback<T> for ErrorHandlingCallback<T> {
    fn call(&self, doc_id: usize, result: Result<T, Option<String>>) {
        let _ = self.0.send((doc_id, result));
    }
}

pub struct ErrorHandlingProvider<D> {
    inner: Arc<Provider<D>>,
    callback: Arc<dyn NetCallback<D>>,
    pending_requests: PendingCount,
}

impl<D: Send + Sync + 'static> ErrorHandlingProvider<D> {
    pub fn new(callback: Arc<dyn NetCallback<D>>) -> Self {
        let inner = Arc::new(Provider::new(callback.clone()));
        Self {
            inner,
            callback,
            pending_requests: PendingCount::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pending_requests.is_empty()
    }
}

impl<D: Send + Sync + 'static> NetProvider<D> for ErrorHandlingProvider<D> {
    fn fetch(&self, doc_id: usize, request: Request, handler: BoxedHandler<D>) {
        self.pending_requests.increment();

        let callback = self.callback.clone();
        let request_url = request.url.to_string();
        let pending_counter = self.pending_requests.clone();

        self.inner.fetch_with_callback(request, Box::new(move |fetch_result| {
            pending_counter.decrement();

            match fetch_result {
                Ok((_url, bytes)) => {
                    println!("Fetched {}", request_url);
                    handler.bytes(doc_id, bytes, callback);
                }
                Err(e) => {
                    let error_msg = Some(format!("Failed to fetch {}: {:?}", request_url, e));
                    callback.call(doc_id, Err(error_msg));
                }
            }
        }));
    }
}

pub struct NetFetcher {
    provider: Arc<ErrorHandlingProvider<Resource>>,
    receiver: UnboundedReceiver<(usize, Result<Resource, Option<String>>)>,
}

impl NetFetcher {
    pub fn new() -> Self {
        let (receiver, callback) = ErrorHandlingCallback::new();
        let callback = Arc::new(callback);
        let provider = Arc::new(ErrorHandlingProvider::new(callback));

        Self { provider, receiver }
    }

    pub fn get_provider(&self) -> Arc<dyn NetProvider<Resource>> {
        Arc::clone(&self.provider) as Arc<dyn NetProvider<Resource>>
    }

    pub async fn fetch_resources(&mut self, document: &mut HtmlDocument) {
        loop {
            // Synchronous fetch before checking is_empty to avoid race condition
            // where is_empty's reference counting thinks there is nothing to
            // process. This happens when the fetch fails very early.
            let res = match self.receiver.try_recv() {
                Ok((_, res)) => res,
                Err(_) => {
                    if self.provider.is_empty() {
                        break;
                    }

                    match self.receiver.recv().await {
                        Some((_, res)) => res,
                        None => break,
                    }
                }
            };

            match res {
                Ok(res) => document.as_mut().load_resource(res),
                Err(_) => {}
            }
        }
    }
}
