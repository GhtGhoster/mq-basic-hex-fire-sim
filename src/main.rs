
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
    let mut hex_size: f32 = 10.0;

    let mut hex_edge: f32 = 0.0;
    let mut highlight_mouse: bool = true;
    let mut highlight_axes: bool = false;
    let mut highlight_neighbours: bool = false;

    let temp_mod_min: f32 = 0.0;
    let temp_mod_max: f32 = 10.0;
    let mut temperature_modify: f32 = 5.0;

    let mut temperature_add: bool = false;

    let tps_min: f64 = 0.0;
    let tps_max: f64 = 120.0;
    let mut tps: f64 = 30.0;

    let mut last_tick = get_time();
    let mut ticks_last_second: Vec<f64> = vec![];

    let mut last_matrix_index: (isize, isize) = (0, 0);
    let mut last_mouse_left: bool = false;
    let mut last_mouse_right: bool = false;

    let mut matrix: HexMatrix = HexMatrix::new(
        false, // vertical hexagons
        (0, 0), // matrix size
        0.5, // heat loss
        0.5, // heat transfer
    );

    loop {
        // used later for tick counting
        let curr_time =  get_time();
        // used latter for automaton input
        let current_matrix_index: (isize, isize) = matrix.get_mouse_hex_coords(hex_size);

        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("HexTempSim")
                .show(egui_ctx, |ui| {
                    ui.collapsing("Controls", |ui| {
                        ui.label("Right mouse: Add or set temperature to a cell");
                        ui.label("Left mouse: Subtract or remove temperature from a cell");
                        ui.label("Scroll: Zoom in or out (change hexagon size)");
                    });

                    ui.collapsing("Stats", |ui| {
                        ui.label(format!("Cursor location: [{}, {}]", current_matrix_index.0, current_matrix_index.1));
                        ui.label(format!("Neighbours: {}", matrix.neighbour_indices(current_matrix_index).len()));
                        ui.label(format!("FPS: {}", get_fps()));
                        ui.label(format!("TPS: {}", ticks_last_second.len()));
                        ui.horizontal(|ui| {
                            if cfg!(target_arch = "wasm32") && cfg!(target_os = "unknown") {
                                ui.label("Target TPS*:").on_hover_ui(|ui|  {
                                    ui.label("Target TPS will only work in fractions of FPS (for 60 FPS, TPS can be 1, 2, ..., 20, 30, 60)");
                                });
                            } else {
                                ui.label("Target TPS:");
                            }
                            ui.add(egui::Slider::new(&mut tps, tps_min..=tps_max));
                        });
                    });

                    ui.collapsing("Size and rotation", |ui| {
                        ui.checkbox(&mut matrix.hex_vertical, "Vertical hexagons");
                        ui.horizontal(|ui| {
                            ui.label("Hexagon size:");
                            ui.add(egui::Slider::new(&mut hex_size, hex_size_min..=hex_size_max));
                        });
                    });

                    ui.collapsing("Rendering", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Edge %:");
                            ui.add(egui::Slider::new(&mut hex_edge, 0.0..=1.0));
                        });
                        ui.checkbox(&mut highlight_mouse, "Highlight current mouse position");
                        ui.checkbox(&mut highlight_neighbours, "Highlight neighbours of mouse");
                        ui.checkbox(&mut highlight_axes, "Highlight current row and column");
                    });

                    ui.collapsing("Simulation properties", |ui| {
                        ui.checkbox(&mut temperature_add, "Add temperature instead of overwriting");
                        ui.horizontal(|ui| {
                            ui.label("Temp magnitude on click:");
                            ui.add(egui::Slider::new(&mut temperature_modify, temp_mod_min..=temp_mod_max));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Heat loss %:");
                            ui.add(egui::Slider::new(&mut matrix.heat_loss, 0.0..=1.0));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Heat transfer %:");
                            ui.add(egui::Slider::new(&mut matrix.heat_tran, 0.0..=1.0));
                        });
                    });
                }
            );
        });

        // meta input
        hex_size += mouse_wheel().1 / 120.0;
        hex_size = hex_size_max.min(hex_size);
        hex_size = hex_size_min.max(hex_size);
        
        // how many hexagonal rows and columns can fit on screen for given size and orientation
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
            matrix = HexMatrix::new(matrix.hex_vertical, new_matrix_size, matrix.heat_loss, matrix.heat_tran);
        }

        // automaton input
        // positive
        if is_mouse_button_down(MouseButton::Left) {
            if !last_mouse_left {
                last_matrix_index = current_matrix_index;
            }
            for (x, y) in matrix.offset_line(last_matrix_index, current_matrix_index) {
                if matrix.contains_index((x, y)) {
                    let (x, y): (usize, usize) = (x as usize, y as usize);
                    if temperature_add {
                        // Note: should probably be done based on ticks which would require
                        //       restructuring at a scale not worth it for this project
                        matrix.matrix[y][x] += 10f32.powf(temperature_modify);
                    } else {
                        matrix.matrix[y][x] = 10f32.powf(temperature_modify);
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
                        matrix.matrix[y][x] = (matrix.matrix[y][x] - 10f32.powf(temperature_modify)).max(0.0);
                    } else {
                        matrix.matrix[y][x] = 0.0;
                    }
                }
            }
        }

        // ticking simulation
        let web_adjusted_tps = if cfg!(target_arch = "wasm32") && cfg!(target_os = "unknown") {
            tps + 1.0 // since FPS and TPS are fraction-locked somehow, allow the current TPS target to be achieved exactly
        } else {
            tps
        };
        if (curr_time - last_tick) > (1.0 / web_adjusted_tps) {
            matrix.update();
            last_tick = curr_time;
            ticks_last_second.push(curr_time);
        }

        // tick counting
        for i in (0..ticks_last_second.len()).rev() {
            let tick_time = ticks_last_second[i];
            if curr_time - tick_time > 1.0 {
                ticks_last_second.remove(i);
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
                // highlight neighbours
                if (
                    highlight_neighbours &&
                    matrix.neighbour_indices((i as isize, j as isize)).contains(&(current_matrix_index.0 as usize, current_matrix_index.1 as usize))
                ) || (
                    highlight_axes && (
                        current_matrix_index.0 == i as isize ||
                        current_matrix_index.1 == j as isize
                    )
                ) || (
                    highlight_mouse &&
                    current_matrix_index.0 == i as isize &&
                    current_matrix_index.1 == j as isize
                ) {
                    draw_hexagon(
                        x,
                        y,
                        hex_size,
                        0.0,
                        matrix.hex_vertical,
                        BLACK,
                        WHITE,
                    );
                }
                draw_hexagon(
                    x,
                    y,
                    hex_size * (1.0 - hex_edge),
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
