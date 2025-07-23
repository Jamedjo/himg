use blitz_html::HtmlDocument;
use blitz_net::{MpscCallback, Provider};
use blitz_dom::net::Resource;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct NetFetcher {
    provider: Arc<Provider<Resource>>,
    receiver: UnboundedReceiver<(usize, Resource)>,
}

impl NetFetcher {
    pub fn new() -> Self {
        let (receiver, callback) = MpscCallback::new();
        let callback = Arc::new(callback);
        let provider = Arc::new(Provider::new(callback));

        Self { provider, receiver }
    }

    pub fn get_provider(&self) -> Arc<Provider<Resource>> {
        Arc::clone(&self.provider)
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
            document.as_mut().load_resource(res);
        }
    }
}
