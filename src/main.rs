
use macroquad::prelude::*;

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

    let mut matrix_size: (usize, usize) = (0, 0);

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

        // input
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

        if new_matrix_size != matrix_size {
            let new_matrix_width: usize = new_matrix_size.0;
            let new_matrix_height: usize = new_matrix_size.1;
            println!("[{new_matrix_width}, {new_matrix_height}]");
            // reinitiate array
            matrix_size = new_matrix_size;
        }

        // rendering
        clear_background(DARKGRAY);
        for i in 0..matrix_size.0 {
            for j in 0..matrix_size.1 {
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
                draw_hexagon(
                    x, //screen_width()/2.0,
                    y, //screen_height()/2.0,
                    hex_size * (1.0 - hex_edge), //(screen_width()/2.0).min(screen_height()/2.0),
                    0.0,
                    hex_vertical,
                    Color::from_rgba(0, 0, 0, 0),
                    Color::from_rgba(0, 0, 0, 255)
                );
            }
        }

        egui_macroquad::draw();
        next_frame().await
    }
}
