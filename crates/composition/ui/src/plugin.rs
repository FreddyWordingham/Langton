use arc_button::ButtonPlugin;
use arc_spawn::SpawnPlugin;
use arc_time::TimePlugin;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ButtonPlugin, SpawnPlugin, TimePlugin));
    }
}
