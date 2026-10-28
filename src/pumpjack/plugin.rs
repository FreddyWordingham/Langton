use bevy::prelude::*;

use crate::pumpjack::{PumpjackFixedUpdateSet, create_items};

pub(crate) struct PumpjackPlugin;

impl Plugin for PumpjackPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, PumpjackFixedUpdateSet::Create)
            .add_systems(FixedUpdate, create_items.in_set(PumpjackFixedUpdateSet::Create));
    }
}
