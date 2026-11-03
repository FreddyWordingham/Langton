use std::marker::PhantomData;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::traits::TilemapLayer;

#[derive(Resource)]
pub(crate) struct TilemapChunks<L: TilemapLayer> {
    chunks: HashMap<UVec2, Entity>,
    layer: PhantomData<fn() -> L>,
}

impl<L: TilemapLayer> TilemapChunks<L> {
    pub(crate) fn new(chunks: HashMap<UVec2, Entity>) -> Self {
        Self {
            chunks,
            layer: PhantomData,
        }
    }

    pub(crate) fn get(&self, chunk_coord: UVec2) -> Entity {
        self.chunks[&chunk_coord]
    }
}
