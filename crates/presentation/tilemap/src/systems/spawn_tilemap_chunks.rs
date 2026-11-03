use arc_grid::iter_coords;
use bevy::{
    image::ImageLoaderSettings,
    platform::collections::HashMap,
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

use crate::{
    components::TilemapChunkMarker,
    resources::{TilemapChunks, TilemapSettings},
    traits::TilemapLayer,
};

pub(crate) fn spawn_tilemap_chunks<L: TilemapLayer>(
    mut commands: Commands,
    assets: Res<AssetServer>,
    tilemap_settings: Res<TilemapSettings<L>>,
    source: Res<L::Source>,
) {
    let config = tilemap_settings.config();
    let array_layout = config.array_layout;
    let sampler = config.sampler.clone();
    let tileset = assets
        .load_builder()
        .with_settings(move |settings: &mut ImageLoaderSettings| {
            settings.array_layout = array_layout;
            settings.sampler = sampler.clone();
        })
        .load(config.tileset_path.clone());

    let grid_size = L::size(&source);
    let mut ents = HashMap::new();
    let num_chunks = UVec2::new(
        grid_size.x.div_ceil(config.chunk_size.x),
        grid_size.y.div_ceil(config.chunk_size.y),
    );
    for chunk_coord in iter_coords(num_chunks) {
        let origin = chunk_coord * config.chunk_size;
        let chunk_size = config.chunk_size.min(grid_size - origin);
        let tile_data: Vec<Option<TileData>> = iter_coords(chunk_size)
            .map(|local_coord| L::tile_data(&source, origin + local_coord))
            .collect();

        let center = (origin.as_vec2() + chunk_size.as_vec2() * 0.5) * config.tile_display_size.as_vec2();
        let ent = commands
            .spawn((
                TilemapChunkMarker::<L>::new(),
                TilemapChunk {
                    chunk_size,
                    tile_display_size: config.tile_display_size,
                    tileset: tileset.clone(),
                    alpha_mode: config.alpha_mode,
                },
                TilemapChunkTileData(tile_data),
                Transform::from_translation(center.extend(config.z_index)),
            ))
            .id();
        ents.insert(chunk_coord, ent);
    }
    commands.insert_resource(TilemapChunks::<L>::new(ents));
}
