use blitz_html::HtmlDocument;
use blitz_net::{MpscCallback, Provider};
use blitz_dom::DocumentConfig;
use anyrender_vello_cpu::VelloCpuImageRenderer;
use anyrender::render_to_buffer;
use blitz_paint::paint_scene;
use blitz_traits::shell::{Viewport};
use std::sync::Arc;

use crate::image_size::ImageSize;
use crate::logger::Logger;
use crate::options::Options;

pub struct RenderOutput {
    pub buffer: Vec<u8>,
    pub image_size: ImageSize,
}

pub async fn html_to_image(
    html: &str,
    options: Options,
    logger: &mut dyn Logger,
) -> RenderOutput {
    let (mut recv, callback) = MpscCallback::new();
    logger.log("Initial config");

    let callback = Arc::new(callback);
    let net = Arc::new(Provider::new(callback));
    logger.log("Setup blitz-net Provider");

    logger.log("Setup dummy navigation provider");

    // Create HtmlDocument
    let mut document = HtmlDocument::from_html(
        &html,
        DocumentConfig {
            base_url: options.base_url,
            net_provider: Some(Arc::clone(&net) as _),
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

    while !net.is_empty() {
        let Some((_, res)) = recv.recv().await else {
            break;
        };
        document.as_mut().load_resource(res);
    }

    logger.log("Fetched assets");

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
