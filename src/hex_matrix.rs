
use macroquad::prelude::{*};

// hex_horizontal
const NEIGHBOUR_INDEX_DELTAS_HORIZONTAL_EVEN: [(isize, isize); 6] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (-1, -1),
    (1, 0),
    (1, -1),
];
const NEIGHBOUR_INDEX_DELTAS_HORIZONTAL_ODD: [(isize, isize); 6] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (-1, 1),
    (1, 0),
    (1, 1),
];
const NEIGHBOUR_INDEX_DELTAS_VERTICAL_EVEN: [(isize, isize); 6] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
    (1, 0),
];
const NEIGHBOUR_INDEX_DELTAS_VERTICAL_ODD: [(isize, isize); 6] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (1, -1),
    (1, 1),
];
pub const HEX_CONST: f32 = 0.8660254; // 3f32.sqrt() / 2f32

pub struct HexMatrix {
    pub hex_vertical: bool,
    pub matrix_size: (usize, usize), // width, height
    pub matrix: Vec<Vec<f32>>,
}

impl HexMatrix {
    pub fn new(hex_vertical: bool, matrix_size: (usize, usize)) -> Self {
        let (matrix_width, matrix_height): (usize, usize) = matrix_size;
        let mut matrix: Vec<Vec<f32>> = Vec::with_capacity(matrix_height);
        for _ in 0..matrix_height {
            matrix.push(vec![0.0; matrix_width]);
        }
        HexMatrix {
            hex_vertical,
            matrix_size,
            matrix,
        }
    }

    pub fn contains_index(&self, (x, y): (isize, isize)) -> bool {
        (0..self.matrix_size.0 as isize).contains(&x) && (0..self.matrix_size.1 as isize).contains(&y)
    }

    pub fn neighbour_indices(&self, index: (isize, isize)) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = vec![];
        let neighbour_index_deltas = if self.hex_vertical {
            if index.1&1 == 0 {
                NEIGHBOUR_INDEX_DELTAS_VERTICAL_EVEN
            } else {
                NEIGHBOUR_INDEX_DELTAS_VERTICAL_ODD
            }
        } else {
            if index.0&1 == 0 {
                NEIGHBOUR_INDEX_DELTAS_HORIZONTAL_EVEN
            } else {
                NEIGHBOUR_INDEX_DELTAS_HORIZONTAL_ODD
            }
        };
        for (x_delta, y_delta) in neighbour_index_deltas {
            let neighbour_x: isize = index.0 + x_delta;
            let neighbour_y: isize = index.1 + y_delta;
            if self.contains_index((neighbour_x, neighbour_y)) {
                result.push((neighbour_x as usize, neighbour_y as usize));
            }
        }
        result
    }

    pub fn get_mouse_hex_coords(&self, hex_size: f32) -> (isize, isize) {
        let (mut mouse_x, mut mouse_y): (f32, f32) = mouse_position();
        if self.hex_vertical {
            mouse_x -= HEX_CONST * hex_size;
            mouse_y -= hex_size;
        } else {
            mouse_x -= hex_size;
            mouse_y -= HEX_CONST * hex_size;
        }
        let (q, r): (f32, f32) = if self.hex_vertical {
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
        let (x, y): (isize, isize) = if self.hex_vertical {
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
    
    pub fn offset_coord_to_center_pixel(&self, hex_size: f32, (i, j): (usize, usize)) -> (f32, f32) { 
        if self.hex_vertical {
            (
                hex_size * HEX_CONST * ((j%2) + 1 + (2*i)) as f32,
                hex_size * ((j as f32 * 1.5) + 1.0) as f32
            )
        } else {
            (
                hex_size * ((i as f32 * 1.5) + 1.0) as f32,
                hex_size * HEX_CONST * ((i%2) + 1 + (2*j)) as f32
            )
        }
    }
}

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