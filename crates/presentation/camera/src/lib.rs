#![allow(clippy::needless_pass_by_value)]

mod components;
mod constants;
mod plugin;
mod resources;
mod sets;
mod systems;

pub use crate::{
    components::MainCamera,
    plugin::CameraPlugin,
    resources::CameraControl,
    sets::{CameraStartupSet, CameraUpdateSet},
};
