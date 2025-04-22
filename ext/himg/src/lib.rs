pub mod renderer;
pub mod html_to_image;
pub mod image_size;
pub mod options;
pub mod writer;
pub mod logger;

pub use renderer::render_blocking;
pub use image_size::ImageSize;
pub use options::Options;
pub use html_to_image::html_to_image;
pub use writer::write_png;

use blitz_traits::ColorScheme;
use magnus::{function, prelude::*, ExceptionClass, Error, Ruby, RString, RHash};

impl Options {
    fn get_option<V: magnus::TryConvert + magnus::IntoValue>(optional_hash: Option<RHash>, key: &str, default: V) -> Result<V, Error> {
        match optional_hash {
            Some(hash) => hash.lookup2::<&str, V, V>(key, default),
            None => Ok(default),
        }
    }

    pub fn from_ruby(hash: Option<RHash>) -> Result<Self, Error> {
        let options = Options {
            image_size: ImageSize {
                width: Self::get_option(hash, "width", 720)?,
                height: Self::get_option(hash, "height", 405)?,
                hidpi_scale: 1.0,
            },
            truncate: Self::get_option(hash, "truncate", true)?,
            verbose: Self::get_option(hash, "verbose", false)?,
            color_scheme: ColorScheme::Light,
            allow_net_requests: true, //TODO: Implement using this
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

    //TODO: Allow optional base_url for resolving linked resources (stylesheets, images, fonts, etc)
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
