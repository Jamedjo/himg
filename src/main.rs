//! Load first CLI argument as a path. Fallback to hardcoded file if no CLI argument is provided.

mod image_size;
mod writer;
mod logger;
mod options;
mod html_to_png;
use crate::html_to_png::html_to_png;
use crate::image_size::ImageSize;
use crate::writer::write_png;
use crate::logger::{Logger, TimedLogger};
use crate::options::Options;

use blitz_traits::{ColorScheme};
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

    let options = Options {
        image_size: ImageSize {
            width: 1200,
            height: 800,
            hidpi_scale: 1.0,
        },
        color_scheme: ColorScheme::Light,
        allow_net_requests: true, //TODO: Implement using this
    };

    let buffer = html_to_png(&html, base_url, options, &mut logger).await;

    // Determine output path, and open a file at that path. TODO: make configurable.
    let out_path = compute_filename(&path_string);
    let mut file = File::create(&out_path).unwrap();

    // Encode buffer as PNG and write it to a file
    write_png(&mut file, &buffer, options.image_size.scaled_width(), options.image_size.scaled_height());

    logger.log("Wrote out png");

    // Log result.
    logger.log_total_time("\nDone");
    println!("Written to {}", out_path.display());
}

fn compute_filename(path_string: &str) -> PathBuf {
    let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = cargo_dir.join("output");

    let base_path = Path::new(path_string).file_stem().unwrap();

    out_dir.join(&base_path).with_extension("png")
}

