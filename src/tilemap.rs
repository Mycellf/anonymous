use macroquad::prelude::*;
use nalgebra::{vector, Vector2};

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
        let view_area = vector![
            screen_width() / camera.zoom.x,
            screen_height() / camera.zoom.y
        ];
    }
}

/// A square array of `Tile`s.
/// * `N` denotes the width and height of the chunk.
#[derive(Clone, Debug)]
pub struct Chunk<const N: usize> {
    pub tiles: [[Tile; N]; N],
}

impl<const N: usize> Chunk<N> {
    pub fn gen_from_tile(tile: Tile) -> Self {
        let tiles = [[tile; N]; N];
        Self { tiles }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {}

impl Tile {
    const SIZE: usize = 8;
}
