use bevy::{
    image::{ImageArrayLayout, ImageSampler},
    prelude::*,
    sprite_render::AlphaMode2d,
};

#[derive(Debug, Clone)]
pub struct TilemapConfig {
    pub tileset_path: String,
    pub array_layout: Option<ImageArrayLayout>,
    pub tile_display_size: UVec2,
    pub chunk_size: UVec2,
    pub z_index: f32,
    pub sampler: ImageSampler,
    pub alpha_mode: AlphaMode2d,
}
