use bevy::prelude::*;

use crate::{
    item::Item,
    render::{RenderSet, sync_appearances},
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, RenderSet::Sync)
            .add_systems(Update, sync_appearances::<Item>.in_set(RenderSet::Sync));
    }
}
