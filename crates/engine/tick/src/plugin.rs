use bevy::prelude::*;

use crate::{resources::Tick, sets::TickFixedUpdate, systems::increment_tick};

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, TickFixedUpdate::Increment)
            .init_resource::<Tick>()
            .add_systems(FixedUpdate, increment_tick.in_set(TickFixedUpdate::Increment));
    }
}
