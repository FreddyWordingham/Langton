use arc_button::ButtonPlugin;
use arc_time::TimePlugin;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ButtonPlugin, TimePlugin));
    }
}
