#![allow(clippy::needless_pass_by_value)]

mod types;
mod utils;

pub use crate::{
    types::{DirtyTrackingGrid, Grid},
    utils::{coord_to_index, index_to_coord, iter_coords},
};
