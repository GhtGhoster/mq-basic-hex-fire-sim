
use macroquad::prelude::*;

use crate::hex_matrix::HexMatrix;

// https://www.redblobgames.com/grids/hexagons/#rounding
pub fn cube_round((q, r): (f32, f32)) -> (isize, isize) {
    let s = -q-r;

    let mut rounded_q = q.round();
    let mut rounded_r = r.round();
    let rounded_s = s.round();

    let q_diff = (rounded_q - q).abs();
    let r_diff = (rounded_r - r).abs();
    let s_diff = (rounded_s - s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        rounded_q = -rounded_r-rounded_s;
    } else if r_diff > s_diff {
        rounded_r = -rounded_q-rounded_s;
    }
    // else {
    //     rounded_s = -rounded_q-rounded_r;
    // }

    (rounded_q as isize, rounded_r as isize)
}

pub fn get_mouse_hex_coords(hex_size: f32, matrix: &HexMatrix) -> (isize, isize) {
    let hex_const: f32 = f32::sqrt(3.0) / 2.0;
    let (mut mouse_x, mut mouse_y): (f32, f32) = mouse_position();
    if matrix.hex_vertical {
        mouse_x -= hex_const * hex_size;
        mouse_y -= hex_size;
    } else {
        mouse_x -= hex_size;
        mouse_y -= hex_const * hex_size;
    }
    let (q, r): (f32, f32) = if matrix.hex_vertical {
        (
            (3f32.sqrt()/3.0 * mouse_x - 1.0/3.0 * mouse_y) / hex_size,
            ((2.0/3.0) * mouse_y) / hex_size,
        )
    } else {
        (
            (2.0/3.0 * mouse_x) / hex_size,
            (-1.0/3.0 * mouse_x + 3f32.sqrt()/3.0 * mouse_y) / hex_size,
        )
    };
    let (q, r): (isize, isize) = cube_round((q, r));
    let (x, y): (isize, isize) = if matrix.hex_vertical {
        (
            (q + (r - (r&1)) / 2) as isize,
            r as isize,
        )
    } else {
        (
            q as isize,
            (r + (q - (q&1)) / 2) as isize,
        )
    };
    (x, y)
}