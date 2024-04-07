use macroquad::prelude::*;

pub struct App {
    camera: Camera2D,
}

impl App {
    pub fn new() -> Self {
        let camera = Camera2D {
            zoom: Vec2::splat(1.0 / 64.0),
            ..Default::default()
        };
        Self { camera }
    }

    pub fn update(&mut self) {}

    pub fn draw(&mut self) {
        self.camera.zoom.x = self.camera.zoom.y * screen_height() / screen_width();
        set_camera(&self.camera);
        draw_rectangle(0.0, 10.0, 5.0, 5.0, WHITE);
    }
}
