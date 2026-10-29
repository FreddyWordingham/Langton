#![allow(clippy::needless_pass_by_value)]

mod constants;
mod plugin;
mod sets;
mod systems;

pub use crate::{plugin::ButtonPlugin, sets::ButtonUpdate};
