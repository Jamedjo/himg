pub mod renderer;
pub mod html_to_image;
pub mod image_size;
pub mod options;
pub mod writer;
pub mod logger;
pub mod net_fetcher;

pub use renderer::render_blocking;
pub use image_size::ImageSize;
pub use options::Options;
pub use html_to_image::html_to_image;
pub use writer::write_png;

use magnus::{function, prelude::*, ExceptionClass, Error, Ruby, RString, RHash};

impl Options {
    pub fn from_ruby(hash: Option<RHash>) -> Result<Self, Error> {
        let defaults = Options::default();

        let hash = match hash {
            None => return Ok(defaults),
            Some(r) => r,
        };

        let options = Options {
            image_size: ImageSize {
                width: hash.lookup2("width", defaults.image_size.width)?,
                height: hash.lookup2("height", defaults.image_size.height)?,
                hidpi_scale: defaults.image_size.hidpi_scale,
            },
            truncate: hash.lookup2("truncate", defaults.truncate)?,
            verbose: hash.lookup2("verbose", defaults.verbose)?,
            base_url: hash.lookup("base_url")?,
            disable_fetch: hash.lookup2("disable_fetch", defaults.disable_fetch)?,
            fetch_timeout: hash.lookup2("fetch_timeout", defaults.fetch_timeout)?,
            color_scheme: defaults.color_scheme,
        };

        Ok(options)
    }
}

pub fn render_blocking_rb(ruby: &Ruby, html: String, options: Option<RHash>) -> Result<RString, Error> {
    let options = Options::from_ruby(options)?;
    let exception_class = ExceptionClass::from_value(magnus::eval("Himg::Error").unwrap()).unwrap();

    match render_blocking(html, options) {
        Ok(data) => Ok(ruby.str_from_slice(&data)),
        Err(e) => Err(Error::new(exception_class, format!("{}", e))),
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Himg")?;

    module.define_singleton_method("render_to_string", function!(render_blocking_rb, 2))?;

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
