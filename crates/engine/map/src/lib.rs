#![allow(clippy::needless_pass_by_value)]

mod messages;
mod plugin;
mod resources;
mod sets;
mod systems;

pub use crate::{
    messages::{CreateMap, DestroyMap},
    plugin::MapPlugin,
    resources::Map,
    sets::MapFixedUpdate,
};
