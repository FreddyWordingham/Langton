use bevy::prelude::*;

use crate::render::Appearance;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    IronOre,
    Methane,
}

impl Item {
    pub(crate) fn is_gas(&self) -> bool {
        matches!(self, Item::Methane)
    }
}

impl Appearance for Item {
    fn sprite(&self) -> &'static str {
        match self {
            Item::IronOre => "sprites/item/solid_raw.png",
            Item::Methane => "sprites/item/gas.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Item::IronOre => Color::srgba(0.5, 0.5, 0.5, 0.5),
            Item::Methane => Color::srgba(0.0, 1.0, 1.0, 0.5),
        }
    }
}
