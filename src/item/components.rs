use bevy::prelude::*;

use crate::lifecycle::ApplyTo;

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
