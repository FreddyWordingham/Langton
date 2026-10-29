#![allow(clippy::needless_pass_by_value)]

mod plugin;
mod resources;
mod sets;
mod systems;

pub use crate::{plugin::TickPlugin, resources::Tick, sets::TickFixedUpdate};
