use macroquad::prelude::*;

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self) {
        draw_text("Hello World", 10.0, 50.0, 40.0, WHITE);
    }
}
