use crate::tilemap::TileMap;
use macroquad::prelude::*;
use nalgebra::vector;

pub struct App {
    camera: Camera2D,
    tilemap: TileMap<16>,
}

impl App {
    pub fn new() -> Self {
        let camera = Camera2D {
            zoom: Vec2::splat(1.0 / 8.0),
            ..Default::default()
        };
        let tilemap = TileMap::gen_from_size(vector![16, 16]);
        Self { camera, tilemap }
    }

    pub fn update(&mut self) {}

    pub fn draw(&mut self) {
        self.camera.zoom.x = self.camera.zoom.y * screen_height() / screen_width();
        set_camera(&self.camera);
        self.tilemap.draw_around(&self.camera);
    }
}
