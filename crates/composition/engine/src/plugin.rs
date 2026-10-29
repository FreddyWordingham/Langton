use arc_core::CorePlugin;
use bevy::prelude::*;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CorePlugin);
    }
}
