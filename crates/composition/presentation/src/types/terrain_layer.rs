use arc_draw::{TILE_SIZE_UVEC2, Tile};
use arc_map::Map;
use arc_tilemap::{TilemapConfig, TilemapLayer};
use bevy::{
    image::{ImageArrayLayout, ImageSampler},
    prelude::*,
    sprite_render::{AlphaMode2d, TileData},
};

pub(crate) struct TerrainLayer;

impl TilemapLayer for TerrainLayer {
    type Source = Map;

    fn size(source: &Self::Source) -> UVec2 {
        source.terrain().size()
    }

    fn tile_data(source: &Self::Source, coord: UVec2) -> Option<TileData> {
        source.terrain()[coord].map(|terrain| terrain.tile_data(coord))
    }

    fn is_dirty(source: &Self::Source) -> bool {
        source.terrain().is_dirty()
    }

    fn drain_dirty(source: &mut Self::Source) -> Vec<UVec2> {
        source.terrain_mut().drain_dirty().collect()
    }
}

impl TerrainLayer {
    pub(crate) fn config() -> TilemapConfig {
        TilemapConfig {
            chunk_size: UVec2::splat(32),
            tile_display_size: TILE_SIZE_UVEC2,
            tileset_path: "atlases/terrain.png".to_owned(),
            array_layout: Some(ImageArrayLayout::GridSize {
                tile_height_pixels: 32,
                tile_width_pixels: 32,
            }),
            z_index: 0.0,
            sampler: ImageSampler::nearest(),
            alpha_mode: AlphaMode2d::Mask(0.5),
        }
    }
}
