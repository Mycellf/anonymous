use macroquad::prelude::*;

pub(crate) mod app;

#[macroquad::main("Anonymous")]
async fn main() {
    let app = app::App::new();

    loop {
        app.draw();
        next_frame().await;
    }
}
