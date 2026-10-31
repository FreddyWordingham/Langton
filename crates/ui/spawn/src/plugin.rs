use bevy::prelude::*;

use crate::{
    resources::ShowSpawnUi,
    sets::SpawnUpdate,
    systems::{despawn_ui, spawn_ui, update_spawn_selection_buttons},
};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (SpawnUpdate::Spawn, SpawnUpdate::Sync, SpawnUpdate::Despawn).chain())
            .insert_resource(ShowSpawnUi::construct())
            .add_systems(
                Update,
                (
                    spawn_ui.run_if(resource_changed::<ShowSpawnUi>).in_set(SpawnUpdate::Spawn),
                    update_spawn_selection_buttons
                        .run_if(resource_changed::<ShowSpawnUi>)
                        .in_set(SpawnUpdate::Sync),
                    despawn_ui
                        .run_if(resource_removed::<ShowSpawnUi>)
                        .in_set(SpawnUpdate::Despawn),
                ),
            );
    }
}
