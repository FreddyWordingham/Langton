#![allow(clippy::needless_pass_by_value)]

mod components;
mod plugin;
mod resources;
mod sets;
mod systems;
mod traits;
mod types;

pub use crate::{plugin::TilemapPlugin, sets::TilemapUpdateSet, traits::TilemapLayer, types::TilemapConfig};
