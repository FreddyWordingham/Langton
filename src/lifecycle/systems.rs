use bevy::prelude::*;

use crate::{
    lifecycle::{ApplyTo, Despawn, SpawnThing},
    spatial::{Position, SpatialIndex},
};

pub(super) fn handle_spawn_thing<T: ApplyTo>(
    mut commands: Commands,
    mut spawn_thing_msg: MessageReader<SpawnThing<T>>,
    mut spatial_index: ResMut<SpatialIndex<T>>,
) {
    for msg in spawn_thing_msg.read() {
        if spatial_index.get(&msg.location).is_none() {
            let mut ent = commands.spawn(Position(msg.location));
            msg.thing.apply_to(&mut ent);
            spatial_index.insert(ent.id(), msg.location);
        } else {
            warn!("Attempted to spawn item at occupied position: {:?}", msg.location);
        }
    }
}

pub(super) fn despawn_marked<T: Component>(
    mut commands: Commands,
    mut spatial_index: ResMut<SpatialIndex<T>>,
    despawn_q: Query<(Entity, &Position), (With<T>, With<Despawn>)>,
) {
    for (ent, pos) in &despawn_q {
        commands.entity(ent).despawn();
        spatial_index.remove(pos.0);
    }
}
