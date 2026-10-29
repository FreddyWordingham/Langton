use bevy::prelude::*;

use crate::{
    resources::CameraControl,
    sets::{CameraStartup, CameraUpdate},
    systems::{pan_camera, spawn_main_camera, zoom_camera},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, CameraStartup::Spawn)
            .configure_sets(Update, CameraUpdate::Apply)
            .init_resource::<CameraControl>()
            .add_systems(Startup, spawn_main_camera.in_set(CameraStartup::Spawn))
            .add_systems(Update, (pan_camera, zoom_camera).in_set(CameraUpdate::Apply));
    }
}
