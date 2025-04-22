use crate::html_to_image::html_to_image;
use crate::image_size::ImageSize;
use crate::options::Options;
use crate::writer::write_png;
use crate::logger::{TimedLogger};

use blitz_traits::{ColorScheme};

pub fn render_blocking(html: String) -> Result<Vec<u8>, std::io::Error> {
    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(render(html))
}

pub async fn render(html: String) -> Result<Vec<u8>, std::io::Error> {
    let mut logger = TimedLogger::init();

    // Configure viewport dimensions
    let options = Options {
        image_size: ImageSize {
            width: 720, //TODO: pass this in
            height: 405, //TODO: decide if this will be fixed or dynamic from the document
            hidpi_scale: 1.0,
        },
        color_scheme: ColorScheme::Light,
        allow_net_requests: true, //TODO: Implement using this
    };

    // Render to Image
    //let base_url = format!("file://{}", path_string.clone());
    let base_url = None;
    let render_output = html_to_image(&html, base_url, options, &mut logger).await;

    // Determine output path, and open a file at that path.
    let mut output_buffer: Vec<u8> = Vec::new();

    // Encode buffer as PNG and write it to a file
    write_png(&mut output_buffer, &render_output.buffer, render_output.image_size.scaled_width(), render_output.image_size.scaled_height())?;

    Ok(output_buffer)
}
