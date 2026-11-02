#![allow(clippy::needless_pass_by_value)]

mod plugin;
mod resources;
mod sets;
mod systems;

pub use crate::{
    plugin::MapPlugin,
    resources::{Tick, TickRate},
    sets::{TickFirst, TickFixedUpdate, TickStartup},
};
