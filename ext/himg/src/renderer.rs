use crate::html_to_image::html_to_image;
use crate::options::Options;
use crate::writer::write_png;
use crate::logger::{Logger, NullLogger, TimedLogger};
use blitz_dom::FontContext;

// render_to_bytes, render_to_string, render_to_file, render_to_io
pub async fn render(html: String, options: Options, font_ctx: Option<FontContext>) -> Result<Vec<u8>, std::io::Error> {
    let mut logger: Box<dyn Logger> = if options.verbose {
        Box::new(TimedLogger::init())
    } else {
        Box::new(NullLogger{})
    };

    // Render to Image
    let render_output = html_to_image(&html, options, &mut *logger, font_ctx).await;

    // Determine output path, and open a file at that path.
    let mut output_buffer: Vec<u8> = Vec::new();

    // Encode buffer as PNG and write it to a file
    write_png(&mut output_buffer, &render_output.buffer, render_output.image_size.scaled_width(), render_output.image_size.scaled_height())?;

    Ok(output_buffer)
}
