use arc_tick::TickRate;
use bevy::prelude::*;

use crate::{
    sets::{TimeStartup, TimeUpdate},
    systems::{spawn_ui, swap_buttons},
};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, TimeStartup::Spawn)
            .configure_sets(Update, TimeUpdate::Sync)
            .add_systems(Startup, spawn_ui.in_set(TimeStartup::Spawn))
            .add_systems(
                Update,
                swap_buttons.run_if(resource_changed::<TickRate>).in_set(TimeUpdate::Sync),
            );
    }
}
