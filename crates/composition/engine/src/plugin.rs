use arc_spatial::SpatialPlugin;
use arc_tick::TickPlugin;
use bevy::prelude::*;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpatialPlugin, TickPlugin));
    }
}
