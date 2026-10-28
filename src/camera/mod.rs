mod components;
mod constants;
mod plugin;
mod sets;
mod systems;

pub(crate) use components::MainCamera;
use constants::{
    CLEAR_COLOR, PAN_EAST_KEY, PAN_NORTH_KEY, PAN_SOUTH_KEY, PAN_SPEED, PAN_WEST_KEY, ZOOM_IN_KEY, ZOOM_OUT_KEY, ZOOM_SPEED,
};
pub(crate) use plugin::CameraPlugin;
pub(crate) use sets::{CameraStartupSet, CameraUpdateSet};
use systems::{pan_main_camera, spawn_main_camera, zoom_main_camera};
