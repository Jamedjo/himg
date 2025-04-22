//! Load first CLI argument as a path. Fallback to hardcoded file if no CLI argument is provided.

use himg::{html_to_image, Options, ImageSize, write_png};
use himg::logger::{Logger, TimedLogger};

use blitz_traits::{ColorScheme};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[tokio::main]
async fn main() {
    let mut logger = TimedLogger::init();

    let path_string = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "./ext/himg/examples/assets/github_profile.html".into());
    println!("Loading {}", path_string);

    // Fetch HTML from path
    let html = std::fs::read_to_string(&path_string).unwrap();
    logger.log("Fetched HTML");

    // Configure viewport dimensions
    let options = Options {
        image_size: ImageSize {
            width: 1200,
            height: 800,
            hidpi_scale: 1.0,
        },
        color_scheme: ColorScheme::Light,
        allow_net_requests: true, //TODO: Implement using this
    };

    // Render to Image
    let base_url = format!("file://{}", path_string.clone());
    let buffer = html_to_image(&html, Some(base_url), options, &mut logger).await;

    // Determine output path, and open a file at that path.
    let out_path = compute_filename(&path_string);
    let mut file = File::create(&out_path).unwrap();

    // Encode buffer as PNG and write it to a file
    write_png(&mut file, &buffer, options.image_size.scaled_width(), options.image_size.scaled_height()).unwrap();
    logger.log("Wrote out png");

    logger.log_total_time("\nDone");
    println!("Written to {}", out_path.display());
}

fn compute_filename(path_string: &str) -> PathBuf {
    let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = cargo_dir.join("output");

    let base_path = Path::new(path_string).file_stem().unwrap();

    out_dir.join(&base_path).with_extension("png")
}

