use macroquad::prelude::*;
use nalgebra::{vector, UnitComplex, Vector2};

/// A dynamic grid of `Chunk`s
/// * `N` denotes the width and height of each chunk.
/// * `size` is the size of the `TileMap` in chunks.
#[derive(Clone, Debug)]
pub struct TileMap<const N: usize> {
    pub size: Vector2<usize>,
    chunks: Vec<Chunk<N>>,
    tile_atlas: Texture2D,
}

impl<const N: usize> TileMap<N> {
    pub fn gen_from_size(size: Vector2<usize>, tile_atlas: Texture2D) -> Self {
        let chunks = std::iter::repeat(Chunk::gen_from_tile(Tile { atlas_index: 0 }))
            .take(size.x * size.y)
            .collect();
        Self {
            size,
            chunks,
            tile_atlas,
        }
    }

    pub fn get_tile(&self, position: Vector2<usize>) -> Tile {
        let (chunk, tile) = self.get_chunk_coords(position);
        self.get_chunk(chunk).tiles[tile.x][tile.y]
    }

    pub fn get_tile_mut(&mut self, position: Vector2<usize>) -> &mut Tile {
        let (chunk, tile) = self.get_chunk_coords(position);
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

    /// Returns (chunk, tile).
    fn get_chunk_coords(&self, position: Vector2<usize>) -> (Vector2<usize>, Vector2<usize>) {
        assert!(position.x < self.size.x && position.y < self.size.y);
        fn rem(vector: Vector2<usize>, scalar: usize) -> Vector2<usize> {
            vector![vector.x % scalar, vector.y % scalar]
        }

        (position / N, rem(position, N))
    }

    pub fn draw_around(&self, camera: &Camera2D, debug_display: bool) {
        // Wasteful when camera is rotated at oblique angles.
        // Change if performance becomes an issue.
        let [horizontal_range, vertical_range] = self.get_area_around(camera);
        for y in vertical_range {
            let offset = y * self.size.x;
            for x in horizontal_range.clone() {
                let i = x + offset;
                let position = vector![x as f32, y as f32] * Chunk::<N>::WORLD_SIZE;
                self.chunks[i].draw_at(position, &self.tile_atlas);
                if debug_display {
                    self.chunks[i].draw_debug_at(position);
                }
            }
        }

        if debug_display {
            let size = vector![self.size.x as f32, self.size.y as f32] * Chunk::<N>::WORLD_SIZE;
            draw_rectangle_lines(0.0, 0.0, size.x, size.y, 0.1, RED);
        }
    }

    fn get_area_around(&self, camera: &Camera2D) -> [std::ops::Range<usize>; 2] {
        use std::f32::consts::PI;
        let center: Vector2<_> = camera.target.into();

        let angle = UnitComplex::new(camera.rotation * PI / 180.0);
        let (cos_mul, sin_mul) = (angle.re.abs(), angle.im.abs());

        let view_area = vector![1.0 / camera.zoom.x, 1.0 / camera.zoom.y,];
        let view_area = vector![
            view_area.x * cos_mul + view_area.y * sin_mul,
            view_area.x * sin_mul + view_area.y * cos_mul,
        ];
        let view_area = [center - view_area, center + view_area];
        let view_area = get_area_in_grid(Chunk::<N>::WORLD_SIZE, self.size, view_area);

        let horizontal_range = view_area[0].x..view_area[1].x;
        let vertical_range = view_area[0].y..view_area[1].y;
        [horizontal_range, vertical_range]
    }
}

/// A square array of `Tile`s.
/// * `N` denotes the width and height of the chunk.
#[derive(Clone, Debug)]
pub struct Chunk<const N: usize> {
    pub tiles: [[Tile; N]; N],
}

impl<const N: usize> Chunk<N> {
    pub const PIXEL_SIZE: usize = N * Tile::PIXEL_SIZE;
    pub const WORLD_SIZE: f32 = N as f32 * Tile::WORLD_SIZE;

    pub fn gen_from_tile(tile: Tile) -> Self {
        let tiles = [[tile; N]; N];
        Self { tiles }
    }

    pub fn draw_at(&self, position: Vector2<f32>, tile_atlas: &Texture2D) {
        for x in 0..N {
            for y in 0..N {
                let tile = self.tiles[x][y];
                if tile.atlas_index == 0 {
                    continue;
                }

                let atlas_position = tile.get_location_in_atlas();
                let world_position = position + vector![x as f32, y as f32] * Tile::WORLD_SIZE;

                draw_texture_ex(
                    &tile_atlas,
                    world_position.x,
                    world_position.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::splat(Tile::WORLD_SIZE)),
                        source: Some(Rect::new(
                            atlas_position.x as f32,
                            atlas_position.y as f32,
                            Tile::PIXEL_SIZE as f32,
                            Tile::PIXEL_SIZE as f32,
                        )),
                        ..Default::default()
                    },
                );
            }
        }
    }

    pub fn draw_debug_at(&self, position: Vector2<f32>) {
        let size = Self::WORLD_SIZE;

        draw_rectangle_lines(position.x, position.y, size, size, 0.05, GREEN);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    pub atlas_index: usize,
}

impl Tile {
    pub const PIXEL_SIZE: usize = 8;
    pub const WORLD_SIZE: f32 = 1.0;

    pub fn new(atlas_index: usize) -> Self {
        Self { atlas_index }
    }

    /// Does not support wrapping for 2D tilemaps
    pub fn get_location_in_atlas(&self) -> Vector2<usize> {
        vector![(self.atlas_index - 1) * Self::PIXEL_SIZE, 0]
    }
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
