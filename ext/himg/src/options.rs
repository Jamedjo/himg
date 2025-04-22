use crate::image_size::ImageSize;

use blitz_traits::ColorScheme;

#[derive(Clone, Copy)]
pub struct Options {
    pub image_size: ImageSize,
    pub color_scheme: ColorScheme,
    pub allow_net_requests: bool,
    pub truncate: bool,
    pub verbose: bool,
}
