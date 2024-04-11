use crate::tilemap::{Tile, TileMap};
use macroquad::prelude::*;
use nalgebra::{vector, UnitComplex};

pub struct App {
    camera: Camera2D,
    tilemap: TileMap<16>,
    debug_display: bool,
}

impl App {
    pub async fn new() -> Self {
        let camera = Camera2D {
            zoom: Vec2::splat(1.0 / 8.0),
            ..Default::default()
        };
        let tile_atlas = load_texture("assets/tile_atlas.png").await.unwrap();
        tile_atlas.set_filter(FilterMode::Nearest);
        let mut tilemap = TileMap::gen_from_size(vector![16, 16], tile_atlas);
        tilemap.set_tile(vector![0, 0], Tile::new(1));
        tilemap.set_tile(vector![1, 0], Tile::new(1));
        tilemap.set_tile(vector![0, 1], Tile::new(1));
        tilemap.set_tile(vector![2, 0], Tile::new(2));
        tilemap.set_tile(vector![1, 1], Tile::new(2));
        tilemap.set_tile(vector![0, 2], Tile::new(2));
        let debug_display = false;
        Self {
            camera,
            tilemap,
            debug_display,
        }
    }

    pub fn update(&mut self) {
        use std::f32::consts::PI;
        self.debug_display ^= is_key_pressed(KeyCode::F3);
        self.camera.target += Vec2::from(
            UnitComplex::new(-self.camera.rotation * PI / 180.0).transform_vector(
                &(vector![
                    (is_key_down(KeyCode::D) as i8 - is_key_down(KeyCode::A) as i8) as f32,
                    (is_key_down(KeyCode::S) as i8 - is_key_down(KeyCode::W) as i8) as f32,
                ] * (16.0 * get_frame_time())),
            ),
        );
        self.camera.rotation += (is_key_down(KeyCode::Q) as i8 - is_key_down(KeyCode::E) as i8)
            as f32
            * (45.0 * get_frame_time());
        if is_key_pressed(KeyCode::R) {
            self.camera.rotation = 0.0;
        }
    }

    pub fn draw(&mut self) {
        self.camera.zoom.x = self.camera.zoom.y * screen_height() / screen_width();
        set_camera(&self.camera);
        self.tilemap.draw_around(&self.camera, self.debug_display);
    }
}
