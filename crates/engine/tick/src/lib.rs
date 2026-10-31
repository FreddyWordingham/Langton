#![allow(clippy::needless_pass_by_value)]

mod constants;
mod plugin;
mod resources;
mod sets;
mod systems;

pub use crate::{
    constants::BASE_TPS,
    plugin::TickPlugin,
    resources::{Tick, TickRate},
    sets::{TickFirst, TickFixedUpdate, TickStartup},
};
