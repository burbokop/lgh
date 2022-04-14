use sdl2::{gfx::primitives::ToColor, pixels::{Color, PixelFormat, PixelFormatEnum}};
use vecmat::Vector;





pub fn map_range_f64(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn add_scalar(value: Vector<f64, 2>, scalar: f64) -> Vector<f64, 2> {
    value.normalize() * (value.length() + scalar)
} 

pub fn map_range_vec_f64(value: f64, in_min: f64 , in_max: f64, out_min: Vector<f64, 2>, out_max: Vector<f64, 2>) -> Vector<f64, 2> {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn contrast (cr_bg: u32) -> u32 {
    const TOLERANCE: i32 = 0x10;
    if (cr_bg as i32 - 0x80).abs() <= TOLERANCE && ((cr_bg >> 8) as i32 - 0x80).abs() <= TOLERANCE && ((cr_bg >> 16) as i32 - 0x80).abs() <= TOLERANCE { 
        (0x7F7F7F + cr_bg) & 0xFFFFFF 
    } else { cr_bg ^ 0xFFFFFF }
}

pub fn contrast_color(c: Color) -> Color {
    Color::from_u32(&PixelFormatEnum::RGBA32.try_into().unwrap(), contrast(c.as_u32()))
}