use bevy::prelude::*;

use crate::{
    render::{RenderUpdateSet, sync_appearances},
    thing::{Item, Machine, Terrain},
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, RenderUpdateSet::Sync).add_systems(
            Update,
            (
                sync_appearances::<Terrain>,
                sync_appearances::<Machine>,
                sync_appearances::<Item>,
            )
                .in_set(RenderUpdateSet::Sync),
        );
    }
}
