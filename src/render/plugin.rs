use bevy::prelude::*;

use crate::render::{RenderSet, sync_meshes};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, RenderSet::Sync)
            .add_systems(Update, sync_meshes.in_set(RenderSet::Sync));
    }
}
