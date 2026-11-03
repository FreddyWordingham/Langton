use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.0;
const TILE_SIZE_U32: u32 = TILE_SIZE as u32;
pub const TILE_SIZE_UVEC2: UVec2 = UVec2::splat(TILE_SIZE_U32);
