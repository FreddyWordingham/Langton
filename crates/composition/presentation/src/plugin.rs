use arc_camera::CameraPlugin;
use bevy::prelude::*;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin);
    }
}
