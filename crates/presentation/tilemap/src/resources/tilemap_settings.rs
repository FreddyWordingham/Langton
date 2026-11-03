use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{traits::TilemapLayer, types::TilemapConfig};

#[derive(Resource)]
pub(crate) struct TilemapSettings<L: TilemapLayer> {
    config: TilemapConfig,
    layer: PhantomData<fn() -> L>,
}

impl<L: TilemapLayer> TilemapSettings<L> {
    pub(crate) fn new(config: TilemapConfig) -> Self {
        Self {
            config,
            layer: PhantomData,
        }
    }

    pub(crate) fn config(&self) -> &TilemapConfig {
        &self.config
    }
}
