use bevy::prelude::*;

use crate::{components::TilemapChunkMarker, resources::TilemapChunks, traits::TilemapLayer};

pub(crate) fn despawn_tilemap_chunks<L: TilemapLayer>(
    mut commands: Commands,
    chunks: Query<Entity, With<TilemapChunkMarker<L>>>,
) {
    for ent in &chunks {
        commands.entity(ent).despawn();
    }
    commands.remove_resource::<TilemapChunks<L>>();
}
