
use macroquad::prelude::*;

pub fn kelvin_to_color(temperature: f32) -> Color {
    // check if I can call powf on a float itself
    // edge case
    if temperature <= 0f32 {
        return BLACK;
    };

    // setup
    let temp = temperature / 100f32;

    let (mut r, mut g, mut b) = if temp <= 66f32 {
        (
            255f32,
            99.4708025861 * f32::ln(temp) - 161.1195681661,
            if temp <= 19f32 {
                0f32
            } else {
                138.5177312231 * f32::ln(temp - 10f32) - 305.0447927307
            }
        )
    } else {
        (
            329.698727466 * f32::powf(temp - 60f32, -0.1332047592),
            288.1221695283 * f32::powf(temp - 60f32, -0.0755148492),
            255f32
        )
    };

    if temperature < 1000f32 {
        let lightness = temperature/1000f32;
        r *= lightness;
        g *= lightness;
        b *= lightness;
    }

    return Color::from_rgba(r as u8, g as u8, b as u8, 255);
}
