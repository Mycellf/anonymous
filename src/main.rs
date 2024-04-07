use macroquad::prelude::*;

mod app;
mod tilemap;

#[macroquad::main("Anonymous")]
async fn main() {
    let mut app = app::App::new();

    loop {
        app.update();
        app.draw();
        next_frame().await;
    }
}
