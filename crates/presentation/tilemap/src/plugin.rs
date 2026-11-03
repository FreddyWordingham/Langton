use std::marker::PhantomData;

use bevy::{prelude::*, sprite_render::update_tilemap_chunk_indices};

use crate::{
    resources::TilemapSettings,
    sets::TilemapUpdateSet,
    systems::{despawn_tilemap_chunks, spawn_tilemap_chunks, sync_tilemap_chunks},
    traits::TilemapLayer,
    types::TilemapConfig,
};

pub struct TilemapPlugin<L: TilemapLayer> {
    config: TilemapConfig,
    layer: PhantomData<fn() -> L>,
}

impl<L: TilemapLayer> TilemapPlugin<L> {
    pub fn new(config: TilemapConfig) -> Self {
        Self {
            config,
            layer: PhantomData,
        }
    }
}

impl<L: TilemapLayer> Plugin for TilemapPlugin<L> {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (TilemapUpdateSet::Spawn, TilemapUpdateSet::Sync, TilemapUpdateSet::Despawn).chain(),
        )
        .insert_resource(TilemapSettings::<L>::new(self.config.clone()))
        .add_systems(
            Update,
            spawn_tilemap_chunks::<L>
                .run_if(resource_added::<L::Source>)
                .in_set(TilemapUpdateSet::Spawn),
        )
        .add_systems(
            Update,
            sync_tilemap_chunks::<L>
                .run_if(resource_exists::<L::Source>)
                .in_set(TilemapUpdateSet::Sync)
                .before(update_tilemap_chunk_indices),
        )
        .add_systems(
            Update,
            despawn_tilemap_chunks::<L>
                .run_if(resource_removed::<L::Source>)
                .in_set(TilemapUpdateSet::Despawn),
        );
    }
}
