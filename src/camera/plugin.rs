use bevy::prelude::*;

use crate::camera::{CLEAR_COLOR, CameraStartupSet, CameraUpdateSet, pan_main_camera, spawn_main_camera, zoom_main_camera};

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, CameraStartupSet::Spawn)
            .configure_sets(Update, CameraUpdateSet::Input)
            .insert_resource(ClearColor(CLEAR_COLOR))
            .add_systems(Startup, spawn_main_camera.in_set(CameraStartupSet::Spawn))
            .add_systems(Update, (pan_main_camera, zoom_main_camera).in_set(CameraUpdateSet::Input));
    }
}
