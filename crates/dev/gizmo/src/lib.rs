#![allow(clippy::needless_pass_by_value)]

mod plugin;
mod sets;
mod systems;

pub use crate::{plugin::GizmoPlugin, sets::GizmoUpdate};
