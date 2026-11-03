use arc_draw::{Draw, Tile};
use bevy::{prelude::*, sprite_render::TileData};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Terrain {
    Sand,
    Soil,
}

impl Terrain {
    pub const ALL: [Self; 2] = [Self::Soil, Self::Sand];
}

impl Tile for Terrain {
    fn tile_data(&self, coord: UVec2) -> TileData {
        match self {
            Self::Sand => TileData {
                tileset_index: ((coord.x % 32) + (((32 - coord.y) % 32) * 32)) as u16,
                ..default()
            },
            Self::Soil => TileData {
                tileset_index: 1024 + ((coord.x % 32) + (((32 - coord.y) % 32) * 32)) as u16,
                ..default()
            },
        }
    }
}

impl Draw for Terrain {
    fn name(&self) -> &'static str {
        match self {
            Self::Soil => "Soil",
            Self::Sand => "Sand",
        }
    }

    fn sprite_path(&self) -> &'static str {
        match self {
            Self::Soil => "sprites/tiles/terrain/misc.png",
            Self::Sand => "sprites/tiles/terrain/misc.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Soil => Color::srgb(0.3, 0.7, 0.4),
            Self::Sand => Color::srgb(0.9, 0.8, 0.5),
        }
    }
}
