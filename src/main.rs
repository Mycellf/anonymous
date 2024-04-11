use macroquad::prelude::*;

mod app;
mod tilemap;

#[macroquad::main("Anonymous")]
async fn main() {
    rand::rand(); // first random number is always zero otherwise
    let mut app = app::App::new().await;

    loop {
        app.update();
        app.draw();
        next_frame().await;
    }
}
