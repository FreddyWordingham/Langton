#![allow(clippy::needless_pass_by_value)]

mod plugin;
mod sets;
mod systems;
mod utils;

pub use crate::{
    plugin::ButtonPlugin,
    sets::ButtonUpdate,
    utils::{button, button_node},
};
