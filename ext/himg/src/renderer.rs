use crate::html_to_image::html_to_image;
use crate::options::Options;
use crate::writer::write_png;
use crate::logger::{TimedLogger};

pub fn render_blocking(html: String, options: Options) -> Result<Vec<u8>, std::io::Error> {
    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(render(html, options))
}

// render_to_bytes, render_to_string, render_to_file, render_to_io
pub async fn render(html: String, options: Options) -> Result<Vec<u8>, std::io::Error> {
    let mut logger = TimedLogger::init();

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
