#![allow(clippy::needless_pass_by_value)]

mod plugin;
mod resources;

pub use crate::{plugin::RandomPlugin, resources::Rng};
