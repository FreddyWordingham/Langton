#![allow(clippy::needless_pass_by_value)]

mod constants;
mod traits;

pub use crate::{
    constants::{TILE_SIZE, TILE_SIZE_UVEC2},
    traits::{Draw, Tile},
};
