
use macroquad::prelude::{*};
use std::mem;

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
    pub buffer: Vec<Vec<f32>>,
    pub heat_loss: f32,
    pub heat_tran: f32,
}

impl HexMatrix {
    pub fn new(hex_vertical: bool, matrix_size: (usize, usize), heat_loss: f32, heat_tran: f32) -> Self {
        let (matrix_width, matrix_height): (usize, usize) = matrix_size;
        let mut matrix: Vec<Vec<f32>> = Vec::with_capacity(matrix_height);
        for _ in 0..matrix_height {
            matrix.push(vec![0.0; matrix_width]);
        }
        let mut buffer: Vec<Vec<f32>> = Vec::with_capacity(matrix_height);
        for _ in 0..matrix_height {
            buffer.push(vec![0.0; matrix_width]);
        }
        HexMatrix {
            hex_vertical,
            matrix_size,
            matrix,
            buffer,
            heat_loss,
            heat_tran,
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
        let (q, r): (isize, isize) = axial_round((q, r));
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

    pub fn offset_to_axial(&self, (x, y): (isize, isize)) -> (isize, isize) {
        if self.hex_vertical {
            (
                x - (y - (y&1)) / 2,
                y,
            )
        } else {
            (
                x,
                y - (x - (x&1)) / 2,
            )
        }
    }

    pub fn axial_to_offset(&self, (q, r): (isize, isize)) -> (isize, isize) {
        if self.hex_vertical {
            (
                q + (r - (r&1)) / 2,
                r,
            )
        } else {
            (
                q,
                r + (q - (q&1)) / 2,
            )
        }
    }

    pub fn axial_distance(&self, (q1, r1): (isize, isize), (q2, r2): (isize, isize)) -> isize {
        (q1-q2).abs() + (q1+r1-q2-r2).abs() + (r1-r2).abs() / 2
    }

    pub fn offset_line(&self, offset_a: (isize, isize), offset_b: (isize, isize)) -> Vec<(isize, isize)> {
        let axial_a: (isize, isize) = self.offset_to_axial(offset_a);
        let axial_b: (isize, isize) = self.offset_to_axial(offset_b);
        let n: isize = self.axial_distance(axial_a, axial_b) + 1;
        let mut result: Vec<(isize, isize)> = Vec::with_capacity(n as usize);
        let float_a: (f32, f32) = (axial_a.0 as f32, axial_a.1 as f32);
        let float_b: (f32, f32) = (axial_b.0 as f32, axial_b.1 as f32);
        for i in 0..n {
            // lerp
            let float_i = (
                float_a.0 + (float_b.0 - float_a.0) * (1.0 / (n as f32) * (i as f32)),
                float_a.1 + (float_b.1 - float_a.1) * (1.0 / (n as f32) * (i as f32)),
            );
            result.push(self.axial_to_offset(axial_round(float_i)));
        }
        result
    }

    pub fn update(&mut self) {
        for x in 0..self.matrix_size.0 {
            for y in 0..self.matrix_size.1 {
                // fading - heat loss
                let mut curr_temp = self.matrix[y][x] * (1.0 - (self.heat_loss / 2.0));
                if curr_temp < 0.01 {
                    curr_temp = 0.0;
                }

                // spreading
                for (nx, ny) in self.neighbour_indices((x as isize, y as isize)) {
                    let mut temp_delta = self.matrix[ny][nx] - self.matrix[y][x];
                    temp_delta *= self.heat_tran / 6.0;
                    curr_temp += temp_delta;
                }

                // writing back
                if self.hex_vertical {
                    // TODO:
                } else {
                    if y > 0 {
                        self.buffer[y-1][x] = curr_temp;
                    } else {
                        self.buffer[self.matrix_size.1-1][x] = 0.0;
                    }
                }
            }
        }
        // swap buffer and matrix
        // (self.matrix, self.buffer) = (self.buffer, self.matrix);
        mem::swap(&mut self.matrix, &mut self.buffer);
    }
}

// https://www.redblobgames.com/grids/hexagons/#rounding
pub fn axial_round((q, r): (f32, f32)) -> (isize, isize) {
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
