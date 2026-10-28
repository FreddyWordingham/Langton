use bevy::prelude::*;

use crate::gas::{GasFixedUpdateSet, move_gas};

pub(crate) struct GasPlugin;

impl Plugin for GasPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, GasFixedUpdateSet::Move)
            .add_systems(FixedUpdate, move_gas.in_set(GasFixedUpdateSet::Move));
    }
}
