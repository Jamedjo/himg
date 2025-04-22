use std::io::Write;
use png::{Encoder, ColorType, BitDepth, PixelDimensions, Unit};

const INCHES_PER_METER: f64 = 39.3701;
const DEFAULT_DPI: f64 = 144.0;

pub fn write_png<W: Write>(writer: W, buffer: &[u8], width: u32, height: u32) -> Result<(), std::io::Error> {
    let encoder = create_encoder(writer, width, height, DEFAULT_DPI);
    write_data(encoder, buffer)
}

fn create_encoder<'a, W: Write>(writer: W, width: u32, height: u32, dpi: f64) -> Encoder<'a, W> {
    let pixels_per_meter = (dpi * INCHES_PER_METER) as u32;

    let mut encoder = Encoder::new(writer, width, height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    encoder.set_pixel_dims(Some(PixelDimensions {
        xppu: pixels_per_meter,
        yppu: pixels_per_meter,
        unit: Unit::Meter,
    }));

    encoder
}

fn write_data<W: Write>(encoder: Encoder<W>, buffer: &[u8]) -> Result<(), std::io::Error> {
    let mut writer = encoder.write_header()?;

    writer.write_image_data(buffer)?;
    writer.finish()?;

    Ok(())
}
