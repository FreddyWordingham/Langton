use bevy::prelude::*;

use crate::pumpjack::PUMPJACK_COOLDOWN;

#[derive(Component)]
pub(crate) struct Pumpjack {
    pub(super) target: Entity,
    pub(super) remaining_cooldown: u8,
}

impl Pumpjack {
    pub(crate) fn new(target: Entity) -> Self {
        Self {
            target,
            remaining_cooldown: PUMPJACK_COOLDOWN,
        }
    }
}
