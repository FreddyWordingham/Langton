use bevy::prelude::*;

use crate::{
    item::Item,
    lifecycle::{LifecycleSet, SpawnThing, despawn_marked, handle_spawn_thing},
};

pub(crate) struct LifecyclePlugin;

impl Plugin for LifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, (LifecycleSet::Spawn, LifecycleSet::Despawn).chain())
            .add_message::<SpawnThing<Item>>()
            .add_systems(
                FixedUpdate,
                (
                    handle_spawn_thing::<Item>.in_set(LifecycleSet::Spawn),
                    despawn_marked::<Item>.in_set(LifecycleSet::Despawn),
                ),
            );
    }
}
