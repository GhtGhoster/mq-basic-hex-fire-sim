
use macroquad::prelude::{*};

mod hex_matrix;
use hex_matrix::HexMatrix;

mod hex_logic;
use hex_logic::{cube_round};

mod color_temperature;
use color_temperature::kelvin_to_color;

#[macroquad::main("mq-basic-hex-fire-sim")]
async fn main() {
    // debugging
    // std::env::set_var("RUST_BACKTRACE", "1");

    let hex_const: f32 = f32::sqrt(3.0) / 2.0;
    let hex_size_min: f32 = 5.0;
    let hex_size_max: f32 = 50.0;

    let mut hex_size: f32 = 10.0;
    let mut hex_vertical: bool = false;
    let mut hex_edge: f32 = 0.0;

    let mut matrix: HexMatrix = HexMatrix::new((0, 0));

    loop {
        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controls")
                .show(egui_ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps()));
                    ui.checkbox(&mut hex_vertical, "Vertical?");
                    ui.horizontal(|ui| {
                        ui.label("Hexagon size:");
                        ui.add(egui::Slider::new(&mut hex_size, hex_size_min..=hex_size_max));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Edge percentage:");
                        ui.add(egui::Slider::new(&mut hex_edge, 0.0..=1.0));
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
        let new_matrix_size: (usize, usize) = if hex_vertical {
            let new_screen_width: f32 = screen_height() - (0.5 * hex_size);
            let new_matrix_width: usize = (new_screen_width / (1.5 * hex_size)).floor() as usize;
            let new_screen_height: f32 = screen_width() - (hex_const * hex_size);
            let new_matrix_height: usize = (new_screen_height / (hex_const * hex_size)).floor() as usize / 2;
            (new_matrix_height, new_matrix_width)
        } else {
            let new_screen_width: f32 = screen_width() - (0.5 * hex_size);
            let new_matrix_width: usize = (new_screen_width / (1.5 * hex_size)).floor() as usize;
            let new_screen_height: f32 = screen_height() - (hex_const * hex_size);
            let new_matrix_height: usize = (new_screen_height / (hex_const * hex_size)).floor() as usize / 2;
            (new_matrix_width, new_matrix_height)
        };

        if new_matrix_size != matrix.matrix_size {
            matrix = HexMatrix::new(new_matrix_size);
        }

        // automaton input
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y): (f32, f32) = mouse_position();
            let (q, r): (f32, f32) = if hex_vertical {
                (
                    (3f32.sqrt()/3.0 * mouse_x) - (1.0/3.0 * mouse_y),
                    2.0/3.0 * mouse_y,
                )
            } else {
                (
                    (2.0/3.0 * mouse_x) / hex_size,
                    ((-1.0/3.0 * mouse_x) + ((3f32).sqrt() * mouse_y)) / hex_size,
                )
            };
            let (q, r): (isize, isize) = cube_round((q, r));
            let (x, y): (usize, usize) = if hex_vertical {
                (
                    (q + (r - (r&1)) / 2) as usize,
                    r as usize,
                )
            } else {
                (
                    (r + (q - (q&1)) / 2) as usize,
                    q as usize,
                )
            };
            //matrix.matrix[x][y] += 1000.0;
        }

        // rendering
        clear_background(DARKGRAY);

        for i in 0..matrix.matrix_size.0 {
            for j in 0..matrix.matrix_size.1 {
                let (x, y): (f32, f32) = if hex_vertical {
                    (
                        hex_size * hex_const * ((j%2) + 1 + (2*i)) as f32,
                        hex_size * ((j as f32 * 1.5) + 1.0) as f32
                    )
                } else {
                    (
                        hex_size * ((i as f32 * 1.5) + 1.0) as f32,
                        hex_size * hex_const * ((i%2) + 1 + (2*j)) as f32
                    )
                };
                let color: Color = kelvin_to_color(matrix.matrix[j][i]);
                draw_hexagon(
                    x, //screen_width()/2.0,
                    y, //screen_height()/2.0,
                    hex_size * (1.0 - hex_edge), //(screen_width()/2.0).min(screen_height()/2.0),
                    0.0,
                    hex_vertical,
                    BLACK,
                    color,
                );
            }
        }

        let (mouse_x, mouse_y): (f32, f32) = mouse_position();
        let (q, r): (f32, f32) = if hex_vertical {
            (
                (3f32.sqrt()/3.0 * mouse_x) - (1.0/3.0 * mouse_y),
                2.0/3.0 * mouse_y,
            )
        } else {
            (
                (2.0/3.0 * mouse_x) / hex_size,
                ((-1.0/3.0 * mouse_x) + ((3f32).sqrt() * mouse_y)) / hex_size,
            )
        };
        let (q, r): (isize, isize) = cube_round((q, r));
        let (j, i): (usize, usize) = if hex_vertical {
            (
                (q + (r - (r&1)) / 2) as usize,
                r as usize,
            )
        } else {
            (
                (r + (q - (q&1)) / 2) as usize,
                q as usize,
            )
        };
        let (x, y): (f32, f32) = if hex_vertical {
            (
                hex_size * hex_const * ((j%2) + 1 + (2*i)) as f32,
                hex_size * ((j as f32 * 1.5) + 1.0) as f32
            )
        } else {
            (
                hex_size * ((i as f32 * 1.5) + 1.0) as f32,
                hex_size * hex_const * ((i%2) + 1 + (2*j)) as f32
            )
        };
        draw_hexagon(x, y, hex_size * (1.0 - hex_edge), 0.0, hex_vertical, BLACK, WHITE);

        egui_macroquad::draw();
        next_frame().await
    }
}
