use crate::image_size::ImageSize;

use blitz_traits::shell::ColorScheme;

#[derive(Clone)]
pub struct Options {
    pub image_size: ImageSize,
    pub color_scheme: ColorScheme,
    pub base_url: Option<String>,
    pub truncate: bool,
    pub verbose: bool,
}

impl Options {
    pub fn default() -> Self {
        Self {
            image_size: ImageSize {
                width: 720,
                height: 405,
                hidpi_scale: 1.0,
            },
            truncate:  true,
            verbose:  false,
            base_url: None,
            color_scheme: ColorScheme::Light,
        }
    }
}
