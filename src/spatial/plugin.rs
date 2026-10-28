use bevy::prelude::*;

use crate::{
    item::Item,
    spatial::{SpatialIndex, SpatialSet, sync_transforms},
};

pub(crate) struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, SpatialSet::Sync)
            .init_resource::<SpatialIndex<Item>>()
            .add_systems(Update, sync_transforms.in_set(SpatialSet::Sync));
    }
}
