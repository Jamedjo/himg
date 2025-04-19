pub mod writer;
pub mod image_size;
pub mod html_to_png;
pub mod logger;
pub mod options;

pub use html_to_png::html_to_png;
pub use image_size::ImageSize;
pub use options::Options;
pub use writer::write_png;
pub use logger::{Logger, TimedLogger};

use magnus::{function, prelude::*, Error, Ruby};

pub fn hello(subject: String) -> String {
    format!("Hello {subject}, from Rust!")
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Himg")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
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
