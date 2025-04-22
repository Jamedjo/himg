pub mod renderer;

pub mod html_to_image;
pub mod image_size;
pub mod options;
pub mod writer;
pub mod logger;

pub use renderer::render_blocking;

use magnus::{function, prelude::*, ExceptionClass, Error, Ruby, RString};

pub fn render_blocking_rb(ruby: &Ruby, html: String) -> Result<RString, Error> {
    let exception_class = ExceptionClass::from_value(magnus::eval("Himg::Error").unwrap()).unwrap();

    match render_blocking(html) {
        Ok(data) => Ok(ruby.str_from_slice(&data)),
        Err(e) => Err(Error::new(exception_class, format!("{}", e))),
    }
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
