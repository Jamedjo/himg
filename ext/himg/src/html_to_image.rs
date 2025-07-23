use blitz_html::HtmlDocument;
use blitz_net::{MpscCallback, Provider};
use blitz_dom::DocumentConfig;
use blitz_dom::net::Resource;
use anyrender_vello_cpu::VelloCpuImageRenderer;
use anyrender::render_to_buffer;
use blitz_paint::paint_scene;
use blitz_traits::shell::{Viewport};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::image_size::ImageSize;
use crate::logger::Logger;
use crate::options::Options;

pub struct RenderOutput {
    pub buffer: Vec<u8>,
    pub image_size: ImageSize,
}

struct NetFetcher {
    provider: Arc<Provider<Resource>>,
    receiver: UnboundedReceiver<(usize, Resource)>,
}

impl NetFetcher {
    fn new() -> Self {
        let (receiver, callback) = MpscCallback::new();
        let callback = Arc::new(callback);
        let provider = Arc::new(Provider::new(callback));

        Self { provider, receiver }
    }

    fn get_provider(&self) -> Arc<Provider<Resource>> {
        Arc::clone(&self.provider)
    }

    async fn fetch_resources(&mut self, document: &mut HtmlDocument) {
        loop {
            // Syncronous fetch before cheking is_empty to avoid race condition
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

pub async fn html_to_image(
    html: &str,
    options: Options,
    logger: &mut dyn Logger,
) -> RenderOutput {
    let mut net_fetcher = if options.disable_fetch {
        logger.log("Disabled fetching resources");

        None
    } else {
        let fetcher = NetFetcher::new();
        logger.log("Setup remote resource fetcher");

        Some(fetcher)
    };

    // Create HtmlDocument
    let mut document = HtmlDocument::from_html(
        &html,
        DocumentConfig {
            base_url: options.base_url,
            net_provider: net_fetcher.as_ref().map(|fetcher| fetcher.get_provider() as _),
            ..Default::default()
        },
    );
    logger.log("Parsed document");

    document.as_mut().set_viewport(Viewport::new(
        options.image_size.scaled_width(),
        options.image_size.scaled_height(),
        options.image_size.hidpi_scale as f32,
        options.color_scheme,
    ));

    if let Some(ref mut net_fetcher) = net_fetcher {
        net_fetcher.fetch_resources(&mut document).await;
        logger.log("Fetched assets");
    }

    // Compute style, layout, etc for HtmlDocument
    document.as_mut().resolve();
    logger.log("Resolved styles and layout");

    // Determine height to render
    let render_size = if options.truncate {
        options.image_size
    } else {
        let computed_height = document.as_ref().root_element().final_layout.size.height;
        let render_height = (computed_height as u32).max(options.image_size.height).min(10_000);
        ImageSize {
            height: render_height,
            ..options.image_size
        }
    };
    logger.log("Calculated render dimensions from document");

    if options.verbose {
        println!("Screenshot is ({}x{})",render_size.scaled_width(), render_size.scaled_height());
    }

    // Render document to RGBA buffer
    let buffer = render_to_buffer::<VelloCpuImageRenderer, _>(
        |scene| paint_scene(
            scene,
            document.as_ref(),
            render_size.hidpi_scale,
            render_size.scaled_width(),
            render_size.scaled_height(),
        ),
        render_size.scaled_width(),
        render_size.scaled_height(),
    );

    logger.log("Rendered to buffer");

    RenderOutput {
        buffer: buffer,
        image_size: render_size,
    }
}
