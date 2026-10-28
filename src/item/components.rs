use bevy::prelude::*;

use crate::lifecycle::ApplyTo;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Item {
    IronOre,
    Methane,
}

impl Item {
    fn color(self) -> Color {
        match self {
            Item::IronOre => Color::srgb(0.5, 0.5, 0.5),
            Item::Methane => Color::srgb(0.0, 1.0, 1.0),
        }
    }
}

impl ApplyTo for Item {
    fn apply_to(self, entity: &mut EntityCommands<'_>) {
        entity.insert(self);
    }
}
