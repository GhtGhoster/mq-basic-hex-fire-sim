
// hex_horizontal
const NEIGHBOUR_INDEX_DELTAS: [(isize, isize); 6] = [
    (0, 1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (1, -1),
    (1, 1),
];

pub struct HexMatrix {
    pub matrix_size: (usize, usize), // width, height
    pub matrix: Vec<Vec<f32>>,
}

impl HexMatrix {
    pub fn new(matrix_size: (usize, usize)) -> Self {
        let (matrix_width, matrix_height): (usize, usize) = matrix_size;
        let mut matrix: Vec<Vec<f32>> = Vec::with_capacity(matrix_height);
        for i in 0..matrix_height {
            matrix.push(vec![0.0; matrix_width]);
            // matrix[i][matrix_width/2] = 6600.0;
            // if i == matrix_height / 2 {
            //     matrix.pop();
            //     matrix.push(vec![6600.0; matrix_width]);
            // }
        }
        HexMatrix {
            matrix_size,
            matrix,
        }
    }

    pub fn neighbour_indices(&self, index: (isize, isize)) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = vec![];
        for (x_delta, y_delta) in NEIGHBOUR_INDEX_DELTAS {
            let neighbour_x: isize = index.0 + x_delta;
            let neighbour_y: isize = index.1 + y_delta;
            if (0..self.matrix_size.0 as isize).contains(&neighbour_x) && (0..self.matrix_size.1 as isize).contains(&neighbour_y) {
                result.push((neighbour_x as usize, neighbour_y as usize));
            }
        }
        result
    }
}
