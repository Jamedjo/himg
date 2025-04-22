pub mod writer;
pub mod image_size;
pub mod html_to_image;
pub mod logger;
pub mod options;

pub use html_to_image::html_to_image;
pub use image_size::ImageSize;
pub use options::Options;
pub use writer::write_png;
pub use logger::{Logger, TimedLogger};

use blitz_traits::{ColorScheme};
use magnus::{function, prelude::*, ExceptionClass, Error, Ruby};

pub fn render_blocking(html: String) -> Result<Vec<u8>, std::io::Error> {
    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(render(html))
}

// render_to_bytes, render_to_string, render_to_file, render_to_io
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
    let image_data = html_to_image(&html, base_url, options, &mut logger).await;

    // Determine output path, and open a file at that path.
    let mut output_buffer: Vec<u8> = Vec::new();

    // Encode buffer as PNG and write it to a file
    write_png(&mut output_buffer, &image_data, options.image_size.scaled_width(), options.image_size.scaled_height())?;

    Ok(output_buffer)
}

pub fn render_blocking_rb(html: String) -> Result<Vec<u8>, magnus::Error> {
    let exception_class = ExceptionClass::from_value(magnus::eval("Himg::Error").unwrap()).unwrap();

    render_blocking(html).map_err(|e| {
        Error::new(exception_class, format!("{}", e))
    })
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Himg")?;

    //TODO: Allow optional base_url for resolving linked resources (stylesheets, images, fonts, etc)
    module.define_singleton_method("render", function!(render_blocking_rb, 1))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rb_sys_test_helpers::ruby_test;
    use super::hello;

    #[ruby_test]
    fn test_hello() {
        assert_eq!("Hello world, from Rust!", hello("world".to_string()));
    }
}
