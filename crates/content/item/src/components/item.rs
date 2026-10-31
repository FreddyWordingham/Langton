use arc_draw::Draw;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    IronOre,
    Methane,
}

impl Item {
    pub const ALL: [Self; 2] = [Self::IronOre, Self::Methane];
}

impl Draw for Item {
    fn name(&self) -> &'static str {
        match self {
            Self::IronOre => "Iron Ore",
            Self::Methane => "Methane",
        }
    }

    fn sprite_path(&self) -> &'static str {
        match self {
            Self::IronOre => "sprites/tiles/item/solid_raw.png",
            Self::Methane => "sprites/tiles/item/gas.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::IronOre => Color::srgb(0.5, 0.5, 1.0),
            Self::Methane => Color::srgb(0.8, 0.6, 0.7),
        }
    }
}
