use bevy::{prelude::*, time::TimeSystems};

use crate::{
    resources::{Tick, TickRate},
    sets::{TickFirst, TickFixedUpdate, TickStartup},
    systems::{increment_tick, init_time, step_tick, update_tps},
};

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, TickStartup::Init)
            .configure_sets(First, TickFirst::Control)
            .configure_sets(FixedUpdate, TickFixedUpdate::Increment)
            .init_resource::<Tick>()
            .init_resource::<TickRate>()
            .add_systems(Startup, init_time.in_set(TickStartup::Init))
            .add_systems(
                First,
                (
                    update_tps.run_if(resource_changed::<TickRate>).before(TimeSystems),
                    step_tick.after(TimeSystems),
                )
                    .in_set(TickFirst::Control),
            )
            .add_systems(FixedUpdate, increment_tick.in_set(TickFixedUpdate::Increment));
    }
}
