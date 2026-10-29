#![allow(clippy::needless_pass_by_value)]

mod components;
mod plugin;
mod sets;
mod systems;
mod utils;

pub use crate::{
    plugin::TimePlugin,
    sets::{TimeStartup, TimeUpdate},
};
