
use macroquad::prelude::{*};

mod hex_matrix;
use hex_matrix::{HexMatrix, HEX_CONST};

mod color_temperature;
use color_temperature::kelvin_to_color;

#[macroquad::main("mq-basic-hex-fire-sim")]
async fn main() {
    // debugging
    // std::env::set_var("RUST_BACKTRACE", "1");

    let hex_size_min: f32 = 5.0;
    let hex_size_max: f32 = 50.0;

    let mut temperature_modify: f32 = 1000.0;
    let mut temperature_add: bool = false;

    let mut last_matrix_index: (isize, isize) = (0, 0);
    let mut last_mouse_left: bool = false;
    let mut last_mouse_right: bool = false;

    let mut hex_size: f32 = 10.0;
    let mut hex_edge: f32 = 0.0;

    let mut matrix: HexMatrix = HexMatrix::new(false, (0, 0));

    loop {
        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controls")
                .show(egui_ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps()));
                    ui.checkbox(&mut matrix.hex_vertical, "Vertical?");
                    ui.horizontal(|ui| {
                        ui.label("Hexagon size:");
                        ui.add(egui::Slider::new(&mut hex_size, hex_size_min..=hex_size_max));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Edge percentage:");
                        ui.add(egui::Slider::new(&mut hex_edge, 0.0..=1.0));
                    });
                    ui.checkbox(&mut temperature_add, "Add temperature instead of overwriting");
                    ui.horizontal(|ui| {
                        ui.label("Temperature on click:");
                        ui.add(egui::Slider::new(&mut temperature_modify, 10.0..=10000.0));
                    });
                }
            );
        });

        // meta input
        hex_size += mouse_wheel().1 / 120.0;
        hex_size = hex_size_max.min(hex_size);
        hex_size = hex_size_min.max(hex_size);
        
        // how many hexagonal rows and columns can fit on screen for given size and orientation
        // TODO: optimize
        let new_matrix_size: (usize, usize) = if matrix.hex_vertical {
            let new_screen_width: f32 = screen_height() - (0.5 * hex_size);
            let new_matrix_width: usize = (new_screen_width / (1.5 * hex_size)).floor() as usize;
            let new_screen_height: f32 = screen_width() - (HEX_CONST * hex_size);
            let new_matrix_height: usize = (new_screen_height / (HEX_CONST * hex_size)).floor() as usize / 2;
            (new_matrix_height, new_matrix_width)
        } else {
            let new_screen_width: f32 = screen_width() - (0.5 * hex_size);
            let new_matrix_width: usize = (new_screen_width / (1.5 * hex_size)).floor() as usize;
            let new_screen_height: f32 = screen_height() - (HEX_CONST * hex_size);
            let new_matrix_height: usize = (new_screen_height / (HEX_CONST * hex_size)).floor() as usize / 2;
            (new_matrix_width, new_matrix_height)
        };

        if new_matrix_size != matrix.matrix_size {
            matrix = HexMatrix::new(matrix.hex_vertical, new_matrix_size);
        }

        // automaton input
        // positive
        // TODO: turn addition tick-based upon addition of TPS
        let current_matrix_index: (isize, isize) = matrix.get_mouse_hex_coords(hex_size);
        if is_mouse_button_down(MouseButton::Left) {
            if !last_mouse_left {
                last_matrix_index = current_matrix_index;
            }
            for (x, y) in matrix.offset_line(last_matrix_index, current_matrix_index) {
                if matrix.contains_index((x, y)) {
                    let (x, y): (usize, usize) = (x as usize, y as usize);
                    if temperature_add {
                        matrix.matrix[y][x] += temperature_modify;
                    } else {
                        matrix.matrix[y][x] = temperature_modify;
                    }
                }
            }
        }
        // negative
        if is_mouse_button_down(MouseButton::Right) {
            if !last_mouse_right {
                last_matrix_index = current_matrix_index;
            }
            for (x, y) in matrix.offset_line(last_matrix_index, current_matrix_index) {
                if matrix.contains_index((x, y)) {
                    let (x, y): (usize, usize) = (x as usize, y as usize);
                    if temperature_add {
                        matrix.matrix[y][x] = (matrix.matrix[y][x] - temperature_modify).max(0.0);
                    } else {
                        matrix.matrix[y][x] = 0.0;
                    }
                }
            }
        }

        last_matrix_index = current_matrix_index;
        last_mouse_left = is_mouse_button_down(MouseButton::Left);
        last_mouse_right = is_mouse_button_down(MouseButton::Right);

        // rendering
        clear_background(DARKGRAY);

        for i in 0..matrix.matrix_size.0 {
            for j in 0..matrix.matrix_size.1 {
                let (x, y): (f32, f32) = matrix.offset_coord_to_center_pixel(hex_size, (i, j));
                let color: Color = kelvin_to_color(matrix.matrix[j][i]);
                draw_hexagon(
                    x, //screen_width()/2.0,
                    y, //screen_height()/2.0,
                    hex_size * (1.0 - hex_edge), //(screen_width()/2.0).min(screen_height()/2.0),
                    0.0,
                    matrix.hex_vertical,
                    BLACK,
                    color,
                );
            }
        }

        egui_macroquad::draw();
        next_frame().await
    }
}
