use bevy::prelude::*;

use crate::{sets::TimeStartup, systems::spawn_ui};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, TimeStartup::Spawn)
            .add_systems(Startup, spawn_ui.in_set(TimeStartup::Spawn));
    }
}
