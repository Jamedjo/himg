//! Load first CLI argument as a path. Fallback to hardcoded file if no CLI argument is provided.

mod image_size;
mod writer;
mod logger;
use image_size::ImageSize;
use writer::write_png;
use crate::logger::{Logger, TimedLogger};

use blitz_dom::net::Resource;
use blitz_html::HtmlDocument;
use blitz_net::{MpscCallback, Provider};
use blitz_renderer_vello::render_to_buffer;
use blitz_traits::navigation::DummyNavigationProvider;
use blitz_traits::net::SharedProvider;
use blitz_traits::{ColorScheme, Viewport};
use std::sync::Arc;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

//struct Himg {
//}

#[tokio::main]
async fn main() {
    let mut logger = TimedLogger::init();

    let path_string = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "/home/james/Software/blitz/examples/assets/github_profile_reduced.html".into());

    println!("{}", path_string);

    // Assert that path is valid
    // TODO

    // Fetch HTML from path
    let file_content = std::fs::read(path_string.clone()).unwrap();
    let base_url = format!("file://{}", path_string.clone());
    let html = String::from_utf8(file_content).unwrap();
    logger.log("Fetched HTML");

    let himg = ImageSize {
        width: 1200,
        height: 800,
        hidpi_scale: 1.0,
    };

    let (mut recv, callback) = MpscCallback::new();
    logger.log("Initial config");

    //let start = std::time::Instant::now();
    //let client = reqwest::Client::new();
    //println!("Client init took {:?}", start.elapsed());

    let callback = Arc::new(callback);
    let net = Arc::new(Provider::new(callback));
    logger.log("Setup blitz-net Provider");

    let navigation_provider = Arc::new(DummyNavigationProvider);

    logger.log("Setup dummy navigation provider");

    // Create HtmlDocument
    let mut document = HtmlDocument::from_html(
        &html,
        Some(base_url),
        Vec::new(),
        Arc::clone(&net) as SharedProvider<Resource>,
        None,
        navigation_provider,
    );

    logger.log("Parsed document");

    let scaled_width = (himg.width as f64 * himg.hidpi_scale as f64) as u32;
    let scaled_height = (himg.height as f64 * himg.hidpi_scale as f64) as u32;
    let scale = himg.hidpi_scale as f32;

    document.as_mut().set_viewport(Viewport::new(
        scaled_width,
        scaled_height,
        scale,
        ColorScheme::Light,
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
    let computed_height = document.as_ref().root_element().final_layout.size.height;
    let render_height = (computed_height as u32).max(himg.height).min(4000);
    let scaled_render_height = (render_height as f64 * himg.hidpi_scale as f64) as u32;

    // Render document to RGBA buffer
    let buffer = render_to_buffer(
        document.as_ref(),
        Viewport::new(
            scaled_width,
            scaled_render_height,
            himg.hidpi_scale,
            ColorScheme::Light,
        ),
    )
    .await;

    logger.log("Rendered to buffer");

    // Determine output path, and open a file at that path. TODO: make configurable.
    let out_path = compute_filename(&path_string);
    let mut file = File::create(&out_path).unwrap();

    // Encode buffer as PNG and write it to a file
    write_png(&mut file, &buffer, scaled_width, scaled_render_height);

    logger.log("Wrote out png");

    // Log result.
    logger.log_total_time("\nDone");
    println!("Screenshot is ({scaled_width}x{scaled_render_height})");
    println!("Written to {}", out_path.display());
}

fn compute_filename(path_string: &str) -> PathBuf {
    let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = cargo_dir.join("output");

    let base_path = Path::new(path_string).file_stem().unwrap();

    out_dir.join(&base_path).with_extension("png")
}

