use arc_draw::Draw;
use arc_item::Item;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    Ore(Item),
    Well(Item),
}

impl Feature {
    pub const ALL: [Self; 2] = [Self::Ore(Item::IronOre), Self::Well(Item::Methane)];
}

impl Draw for Feature {
    fn name(&self) -> &'static str {
        match self {
            Self::Ore(item) => match item {
                Item::IronOre => "Iron Ore",
                _ => "Ore: Unknown",
            },
            Self::Well(item) => match item {
                Item::Methane => "Methane Well",
                _ => "Well: Unknown",
            },
        }
    }

    fn sprite_path(&self) -> &'static str {
        match self {
            Self::Ore(_) => "sprites/tiles/feature/ore.png",
            Self::Well(_) => "sprites/tiles/feature/well.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Ore(item) => item.color(),
            Self::Well(item) => item.color(),
        }
    }
}
