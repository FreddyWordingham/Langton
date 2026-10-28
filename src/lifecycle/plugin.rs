use bevy::prelude::*;

use crate::lifecycle::{
    LifecycleSet, SpawnItem, SpawnMachine, SpawnTerrain, despawn_item, despawn_machine, despawn_terrain, handle_spawn_item,
    handle_spawn_machine, handle_spawn_terrain,
};

pub(crate) struct LifecyclePlugin;

impl Plugin for LifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, (LifecycleSet::Spawn, LifecycleSet::Despawn).chain())
            .add_message::<SpawnTerrain>()
            .add_message::<SpawnMachine>()
            .add_message::<SpawnItem>()
            .add_systems(
                FixedUpdate,
                (
                    (handle_spawn_terrain, handle_spawn_machine, handle_spawn_item).in_set(LifecycleSet::Spawn),
                    (despawn_terrain, despawn_machine, despawn_item).in_set(LifecycleSet::Despawn),
                ),
            );
    }
}
