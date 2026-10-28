use bevy::prelude::*;

use crate::camera::{CLEAR_COLOR, pan_main_camera, spawn_main_camera, zoom_main_camera};

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR))
            .add_systems(Startup, spawn_main_camera)
            .add_systems(Update, (pan_main_camera, zoom_main_camera));
    }
}
