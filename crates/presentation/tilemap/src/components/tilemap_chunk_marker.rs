use std::marker::PhantomData;

use bevy::prelude::*;

use crate::traits::TilemapLayer;

#[derive(Component)]
pub(crate) struct TilemapChunkMarker<L: TilemapLayer>(PhantomData<fn() -> L>);

impl<L: TilemapLayer> TilemapChunkMarker<L> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}
