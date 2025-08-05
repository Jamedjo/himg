pub mod renderer;
pub mod html_to_image;
pub mod image_size;
pub mod options;
pub mod writer;
pub mod logger;
pub mod net_fetcher;

pub use renderer::render;
pub use image_size::ImageSize;
pub use options::Options;
pub use html_to_image::html_to_image;
pub use writer::write_png;

const BULLET_FONT: &[u8] = include_bytes!("../assets/moz-bullet-font.otf");

use std::cell::RefCell;
use std::sync::Arc;
use magnus::{class, function, method, prelude::*, wrap, ExceptionClass, Error, Ruby, RString, RHash};
use tokio::runtime::Runtime;
use blitz_dom::FontContext;
use peniko::Blob;

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
            gpu: hash.lookup2("gpu", defaults.gpu)?,
            color_scheme: defaults.color_scheme,
        };

        Ok(options)
    }
}

#[wrap(class = "Himg::Renderer")]
pub struct Renderer {
    tokio_runtime: RefCell<Runtime>,
    font_ctx: RefCell<FontContext>,
}

impl Renderer {
    pub fn new() -> Result<Self, Error> {
        let mut font_ctx = FontContext::default();
        font_ctx.collection.register_fonts(Blob::new(Arc::new(BULLET_FONT) as _), None);

        let tokio_runtime = Runtime::new()
            .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?;

        Ok(Renderer {
            tokio_runtime: RefCell::new(tokio_runtime),
            font_ctx: RefCell::new(font_ctx),
        })
    }

pub fn render(&self, html: String, options: Option<RHash>) -> Result<RString, Error> {
    let ruby = Ruby::get().unwrap();
    let exception_class = ExceptionClass::from_value(magnus::eval("Himg::Error").unwrap()).unwrap();
    let gpu_not_found_class = ExceptionClass::from_value(magnus::eval("Himg::GpuNotFound").unwrap()).unwrap();

    let options = Options::from_ruby(options)?;

    let font_ctx_clone = self.font_ctx.borrow().clone();
    let tokio_runtime = self.tokio_runtime.borrow();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        tokio_runtime.block_on(
            render(html, options, Some(font_ctx_clone))
        )
    }));

    match result {
        Ok(Ok(data)) => Ok(ruby.str_from_slice(&data)),
        Ok(Err(e)) => Err(Error::new(exception_class, format!("{}", e))),
        Err(panic) => {
            let msg = if let Some(s) = panic.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = panic.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "Unknown panic".to_string()
            };

            if msg.contains("No compatible device found") {
                Err(Error::new(gpu_not_found_class, msg))
            } else {
                Err(Error::new(exception_class, format!("Panic: {}", msg)))
            }
        }
    }
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Himg")?;

    let renderer_class = module.define_class("Renderer", class::object())?;
    renderer_class.define_singleton_method("new", function!(Renderer::new, 0))?;
    renderer_class.define_method("render", method!(Renderer::render, 2))?;

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
