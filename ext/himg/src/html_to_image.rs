use blitz_html::HtmlDocument;
use blitz_dom::DocumentConfig;
use anyrender_vello::VelloImageRenderer;
use anyrender_vello_cpu::VelloCpuImageRenderer;
use anyrender::render_to_buffer;
use blitz_paint::paint_scene;
use blitz_traits::shell::{Viewport};
use tokio::time::{timeout, Duration};

use crate::image_size::ImageSize;
use crate::logger::Logger;
use crate::options::Options;
use crate::net_fetcher::NetFetcher;

pub struct RenderOutput {
    pub buffer: Vec<u8>,
    pub image_size: ImageSize,
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
        match timeout(Duration::from_millis((options.fetch_timeout * 1000.0) as u64), net_fetcher.fetch_resources(&mut document)).await {
            Ok(_) => logger.log("Fetched assets"),
            Err(_) => logger.log("Timeout fetching assets"),
        }
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
    let buffer = if options.gpu {
        render_to_buffer::<VelloImageRenderer, _>(
            |scene| paint_scene(
                scene,
                document.as_ref(),
                render_size.hidpi_scale,
                render_size.scaled_width(),
                render_size.scaled_height(),
            ),
            render_size.scaled_width(),
            render_size.scaled_height(),
        )
    } else {
        render_to_buffer::<VelloCpuImageRenderer, _>(
            |scene| paint_scene(
                scene,
                document.as_ref(),
                render_size.hidpi_scale,
                render_size.scaled_width(),
                render_size.scaled_height(),
            ),
            render_size.scaled_width(),
            render_size.scaled_height(),
        )
    };

    logger.log("Rendered to buffer");

    RenderOutput {
        buffer: buffer,
        image_size: render_size,
    }
}
