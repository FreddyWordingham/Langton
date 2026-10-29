#![allow(clippy::needless_pass_by_value)]

mod constants;
mod plugin;

pub use crate::{constants::TILE_SIZE, plugin::SpatialPlugin};
