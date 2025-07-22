#[derive(Clone, Copy)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
    pub hidpi_scale: f64,
}

impl ImageSize {
    pub fn scaled_width(&self) -> u32 {
        (self.width as f64 * self.hidpi_scale as f64) as u32
    }

    pub fn scaled_height(&self) -> u32 {
        (self.height as f64 * self.hidpi_scale as f64) as u32
    }

    //pub fn scale(&self, value: u32) -> u32 {
    //    (value as f64 * self.hidpi_scale as f64) as u32
    //}
}
