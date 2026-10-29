#![allow(clippy::needless_pass_by_value)]

mod constants;
mod plugin;
mod resources;

pub use crate::{constants::TILE_SIZE, plugin::SpatialPlugin, resources::SpatialIndex};
