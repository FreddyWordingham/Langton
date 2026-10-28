use bevy::prelude::*;

use crate::pumpjack::{PumpjackSet, create_items};

pub(crate) struct PumpjackPlugin;

impl Plugin for PumpjackPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, PumpjackSet::Create)
            .add_systems(FixedUpdate, create_items.in_set(PumpjackSet::Create));
    }
}
