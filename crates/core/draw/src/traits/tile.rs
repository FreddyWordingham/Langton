use bevy::{prelude::*, sprite_render::TileData};

pub trait Tile {
    fn tile_data(&self, coord: UVec2) -> TileData;
}
