use bevy::prelude::*;

use crate::{
    resources::CameraControl,
    sets::{CameraStartupSet, CameraUpdateSet},
    systems::{pan_camera, spawn_main_camera, zoom_camera},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, CameraStartupSet::Spawn)
            .configure_sets(Update, CameraUpdateSet::Apply)
            .init_resource::<CameraControl>()
            .add_systems(Startup, spawn_main_camera.in_set(CameraStartupSet::Spawn))
            .add_systems(Update, (pan_camera, zoom_camera).in_set(CameraUpdateSet::Apply));
    }
}
