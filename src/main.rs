use macroquad::prelude::*;

pub(crate) mod app;

#[macroquad::main("Anonymous")]
async fn main() {
    let app = app::App {};

    loop {
        draw_text("Hello World", 10.0, 50.0, 40.0, WHITE);

        next_frame().await;
    }
}
