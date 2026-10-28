use bevy::prelude::*;

use crate::{
    spatial::{SpatialIndex, SpatialUpdateSet, sync_transforms},
    thing::{Item, Machine, Terrain},
};

pub(crate) struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, SpatialUpdateSet::Sync)
            .init_resource::<SpatialIndex<Terrain>>()
            .init_resource::<SpatialIndex<Machine>>()
            .init_resource::<SpatialIndex<Item>>()
            .add_systems(Update, sync_transforms.in_set(SpatialUpdateSet::Sync));
    }
}
