use blitz_html::HtmlDocument;
use blitz_net::Provider;
use blitz_dom::net::Resource;
use blitz_traits::net::{NetCallback, NetProvider, BoxedHandler, Request};
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};


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
}

impl<D: Send + Sync + 'static> ErrorHandlingProvider<D> {
    pub fn new(callback: Arc<dyn NetCallback<D>>) -> Self {
        let (_, dummy_callback) = ErrorHandlingCallback::new();
        let dummy_callback = Arc::new(dummy_callback);
        let inner = Arc::new(Provider::new(dummy_callback));
        Self {
            inner,
            callback,
        }
    }

    pub fn is_empty(&self) -> bool {
        Arc::strong_count(&self.callback) == 1
    }
}

impl<D: Send + Sync + 'static> NetProvider<D> for ErrorHandlingProvider<D> {
    fn fetch(&self, doc_id: usize, request: Request, handler: BoxedHandler<D>) {
        let callback = self.callback.clone();
        let request_url = request.url.to_string();

        self.inner.fetch_with_callback(request, Box::new(move |fetch_result| {
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
