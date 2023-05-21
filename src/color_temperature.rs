
use macroquad::prelude::*;

pub fn kelvin_to_color(temperature: f32) -> Color {
    // edge case
    if temperature <= 0.0 {
        return BLACK;
    };

    // setup
    let temp = temperature / 100.0;

    let (mut r, mut g, mut b): (f32, f32, f32) = if temp <= 66.0 {
        (
            255.0,
            99.4708025861 * temp.ln() - 161.1195681661,
            if temp <= 19.0 {
                0.0
            } else {
                138.5177312231 * (temp - 10.0).ln() - 305.0447927307
            },
        )
    } else {
        (
            329.698727466 * (temp - 60.0).powf(-0.1332047592),
            288.1221695283 * (temp - 60.0).powf(-0.0755148492),
            255.0,
        )
    };

    if temperature < 1000.0 {
        let lightness = temperature/1000.0;
        r *= lightness;
        g *= lightness;
        b *= lightness;
    }

    return Color::from_rgba(r as u8, g as u8, b as u8, 255);
}
