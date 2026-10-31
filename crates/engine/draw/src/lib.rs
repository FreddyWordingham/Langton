#![allow(clippy::needless_pass_by_value)]

mod plugin;
mod traits;

pub use crate::{plugin::DrawPlugin, traits::Draw};
