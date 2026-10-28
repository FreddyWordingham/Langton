use bevy::prelude::*;

use crate::{lifecycle::ApplyTo, render::Appearance};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Item {
    IronOre,
    Methane,
}

impl ApplyTo for Item {
    fn apply_to(self, entity: &mut EntityCommands<'_>) {
        entity.insert(self);
    }
}

impl Appearance for Item {
    fn sprite(&self) -> &'static str {
        match self {
            Item::IronOre => "sprites/items/ore.png",
            Item::Methane => "sprites/items/gas.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Item::IronOre => Color::srgb(0.5, 0.5, 0.5),
            Item::Methane => Color::srgb(0.0, 1.0, 1.0),
        }
    }
}
