use macroquad::prelude::*;

#[macroquad::main("Anonymous")]
async fn main() {
    loop {
        draw_text("Hello World", 10.0, 50.0, 40.0, WHITE);

        next_frame().await;
    }
}
