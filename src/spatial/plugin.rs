use bevy::prelude::*;

use crate::{
    spatial::{SpatialIndex, SpatialSet, sync_transforms},
    thing::{Item, Machine, Terrain},
};

pub(crate) struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, SpatialSet::Sync)
            .init_resource::<SpatialIndex<Terrain>>()
            .init_resource::<SpatialIndex<Machine>>()
            .init_resource::<SpatialIndex<Item>>()
            .add_systems(Update, sync_transforms.in_set(SpatialSet::Sync));
    }
}
