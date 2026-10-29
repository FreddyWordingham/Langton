use bevy::{prelude::*, time::TimeSystems};

use crate::{
    resources::{Tick, TickRate},
    sets::{TickFirst, TickFixedUpdate, TickUpdate},
    systems::{increment_tick, step_tick, update_tps},
};

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(First, TickFirst::Run.after(TimeSystems))
            .configure_sets(Update, TickUpdate::Update)
            .configure_sets(FixedUpdate, TickFixedUpdate::Increment)
            .init_resource::<Tick>()
            .init_resource::<TickRate>()
            .add_systems(First, step_tick.in_set(TickFirst::Run))
            .add_systems(
                Update,
                update_tps.run_if(resource_changed::<TickRate>).in_set(TickUpdate::Update),
            )
            .add_systems(FixedUpdate, increment_tick.in_set(TickFixedUpdate::Increment));
    }
}
