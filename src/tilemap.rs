use macroquad::prelude::*;
use nalgebra::{vector, DimRange, Vector2};

/// A dynamic grid of `Chunk`s
/// * `N` denotes the width and height of each chunk.
/// * `size` is the size of the `TileMap` in chunks.
#[derive(Clone, Debug)]
pub struct TileMap<const N: usize> {
    pub size: Vector2<usize>,
    chunks: Vec<Chunk<N>>,
}

impl<const N: usize> TileMap<N> {
    pub fn gen_from_size(size: Vector2<usize>) -> Self {
        let chunks = std::iter::repeat(Chunk::gen_from_tile(Tile {}))
            .take(size.x * size.y)
            .collect();
        Self { size, chunks }
    }

    pub fn get_tile(&self, position: Vector2<usize>) -> Tile {
        let (chunk, tile) = Self::get_chunk_coords(position);
        self.get_chunk(chunk).tiles[tile.x][tile.y]
    }

    pub fn get_tile_mut(&mut self, position: Vector2<usize>) -> &mut Tile {
        let (chunk, tile) = Self::get_chunk_coords(position);
        &mut self.get_chunk_mut(chunk).tiles[tile.x][tile.y]
    }

    pub fn set_tile(&mut self, position: Vector2<usize>, tile: Tile) {
        *self.get_tile_mut(position) = tile;
    }

    pub fn get_chunk(&self, chunk: Vector2<usize>) -> &Chunk<N> {
        let index = chunk.x + chunk.y * self.size.x;
        &self.chunks[index]
    }

    pub fn get_chunk_mut(&mut self, chunk: Vector2<usize>) -> &mut Chunk<N> {
        let index = chunk.x + chunk.y * self.size.x;
        &mut self.chunks[index]
    }

    fn get_chunk_coords(position: Vector2<usize>) -> (Vector2<usize>, Vector2<usize>) {
        fn rem(vector: Vector2<usize>, scalar: usize) -> Vector2<usize> {
            vector![vector.x % scalar, vector.y % scalar]
        }

        (position / N, rem(position, N))
    }

    pub fn draw_around(&self, camera: &Camera2D) {
        let chunk_size = Chunk::<N>::get_world_size();
        let center: Vector2<_> = camera.target.into();
        let view_area = vector![1.0 / camera.zoom.x, 1.0 / camera.zoom.y,] / 2.0;
        let view_area = [center - view_area, center + view_area];
        let view_area = get_area_in_grid(chunk_size, self.size, view_area);
        let horizontal_range = view_area[0].x..view_area[1].x;
        let vertical_range = view_area[0].y..view_area[1].y;

        for y in vertical_range {
            let offset = y * self.size.x;
            for x in horizontal_range.clone() {
                let i = x + offset;
                self.chunks[i].draw_at(vector![x as f32, y as f32] * chunk_size);
            }
        }
    }
}

/// A square array of `Tile`s.
/// * `N` denotes the width and height of the chunk.
#[derive(Clone, Debug)]
pub struct Chunk<const N: usize> {
    pub tiles: [[Tile; N]; N],
    pub texture: Texture2D,
}

impl<const N: usize> Chunk<N> {
    pub fn gen_from_tile(tile: Tile) -> Self {
        let tiles = [[tile; N]; N];
        let size = Self::get_pixel_size() as u16;
        let image = Image::gen_image_color(size, size, WHITE);
        let texture = Texture2D::from_image(&image);
        Self { tiles, texture }
    }

    pub fn draw_at(&self, position: Vector2<f32>) {
        let size = Self::get_world_size();

        draw_texture_ex(
            &self.texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::splat(size)),
                ..Default::default()
            },
        );
    }

    pub fn get_pixel_size() -> usize {
        N * Tile::PIXEL_SIZE
    }

    pub fn get_world_size() -> f32 {
        N as f32 * Tile::WORLD_SIZE
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {}

impl Tile {
    const PIXEL_SIZE: usize = 8;
    const WORLD_SIZE: f32 = 1.0;
}

pub fn get_area_in_grid(
    grid_size: f32,
    grid_dim: Vector2<usize>,
    area: [Vector2<f32>; 2],
) -> [Vector2<usize>; 2] {
    fn floor(vector: Vector2<f32>, grid_dim: Vector2<usize>) -> Vector2<usize> {
        vector![
            vector.x.floor().clamp(0.0, grid_dim.x as f32) as usize,
            vector.y.floor().clamp(0.0, grid_dim.y as f32) as usize,
        ]
    }
    fn ceiling(vector: Vector2<f32>, grid_dim: Vector2<usize>) -> Vector2<usize> {
        vector![
            vector.x.ceil().clamp(0.0, grid_dim.x as f32) as usize,
            vector.y.ceil().clamp(0.0, grid_dim.y as f32) as usize,
        ]
    }
    let corners = area.map(|corner| corner / grid_size);
    [floor(corners[0], grid_dim), ceiling(corners[1], grid_dim)]
}