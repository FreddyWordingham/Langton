use arc_draw::Draw;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Terrain {
    Grass,
    Water,
}

impl Terrain {
    pub const ALL: [Self; 2] = [Self::Grass, Self::Water];
}

impl Draw for Terrain {
    fn name(&self) -> &'static str {
        match self {
            Self::Grass => "Grass",
            Self::Water => "Water",
        }
    }

    fn sprite_path(&self) -> &'static str {
        match self {
            Self::Grass => "sprites/tiles/terrain/grass.png",
            Self::Water => "sprites/tiles/terrain/water.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Grass => Color::srgb(0.3, 0.7, 0.4),
            Self::Water => Color::srgb(0.3, 0.4, 0.7),
        }
    }
}
