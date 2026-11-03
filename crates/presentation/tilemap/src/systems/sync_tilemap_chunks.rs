use arc_grid::coord_to_index;
use bevy::{
    prelude::*,
    sprite_render::{TilemapChunk, TilemapChunkTileData},
};

use crate::{
    components::TilemapChunkMarker,
    resources::{TilemapChunks, TilemapSettings},
    traits::TilemapLayer,
};

pub(crate) fn sync_tilemap_chunks<L: TilemapLayer>(
    mut source: ResMut<L::Source>,
    tilemap_settings: Res<TilemapSettings<L>>,
    tilemap_chunks: Res<TilemapChunks<L>>,
    mut tilemap_chunk_tile_data_q: Query<(&mut TilemapChunkTileData, &TilemapChunk), With<TilemapChunkMarker<L>>>,
) {
    if !L::is_dirty(&source) {
        return;
    }

    let dirty_coords: Vec<_> = L::drain_dirty(source.bypass_change_detection());
    let chunk_size = tilemap_settings.config().chunk_size;
    for global_coord in dirty_coords {
        let chunk_coord = global_coord / chunk_size;
        let local_coord = global_coord % chunk_size;
        let entity = tilemap_chunks.get(chunk_coord);
        let (mut tile_data, chunk) = tilemap_chunk_tile_data_q.get_mut(entity).expect("Failed to get chunk entity");

        let index = coord_to_index(local_coord, chunk.chunk_size);
        tile_data[index] = L::tile_data(&source, global_coord);
    }
}
