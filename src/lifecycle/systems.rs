use bevy::prelude::*;

use crate::{
    gas::Gas,
    lifecycle::{Despawn, SpawnItem, SpawnMachine, SpawnTerrain},
    pumpjack::Pumpjack,
    spatial::{Position, SpatialIndex},
    thing::{Item, Machine, Terrain},
};

pub(super) fn handle_spawn_terrain(
    mut commands: Commands,
    mut spawn_terrain_msg: MessageReader<SpawnTerrain>,
    mut terrain_spatial_index: ResMut<SpatialIndex<Terrain>>,
) {
    for msg in spawn_terrain_msg.read() {
        if terrain_spatial_index.get(&msg.location).is_none() {
            let ent = commands.spawn((msg.terrain, Position(msg.location)));
            terrain_spatial_index.insert(ent.id(), msg.location);
        } else {
            warn!("Attempted to spawn terrain at occupied position: {:?}", msg.location);
        }
    }
}

pub(super) fn handle_spawn_machine(
    mut commands: Commands,
    mut spawn_machine_msg: MessageReader<SpawnMachine>,
    terrain_spatial_index: Res<SpatialIndex<Terrain>>,
    mut machine_spatial_index: ResMut<SpatialIndex<Machine>>,
    terrain_q: Query<&Terrain>,
) {
    for msg in spawn_machine_msg.read() {
        if machine_spatial_index.get(&msg.location).is_none() {
            match msg.machine {
                Machine::Drill => {
                    unimplemented!()
                }
                Machine::Pumpjack => {
                    let Some(target_ent) = terrain_spatial_index.get(&msg.location) else {
                        warn!("Attempted to spawn pumpjack at location with no terrain: {:?}", msg.location);
                        continue;
                    };

                    let Ok(terrain) = terrain_q.get(target_ent) else {
                        warn!("Attempted to spawn pumpjack at location with no terrain: {:?}", msg.location);
                        continue;
                    };

                    if !terrain.is_well() {
                        warn!(
                            "Attempted to spawn pumpjack at location with terrain that is not a well: {:?}",
                            msg.location
                        );
                        continue;
                    }

                    let ent = commands.spawn((msg.machine, Position(msg.location), Pumpjack::new(target_ent)));
                    machine_spatial_index.insert(ent.id(), msg.location);
                }
            }
        } else {
            warn!("Attempted to spawn machine at occupied position: {:?}", msg.location);
        }
    }
}

pub(super) fn handle_spawn_item(
    mut commands: Commands,
    mut spawn_item_msg: MessageReader<SpawnItem>,
    mut item_spatial_index: ResMut<SpatialIndex<Item>>,
) {
    for msg in spawn_item_msg.read() {
        if item_spatial_index.get(&msg.location).is_none() {
            let mut ent = commands.spawn((msg.item, Position(msg.location)));
            if msg.item.is_gas() {
                ent.insert(Gas);
            }
            item_spatial_index.insert(ent.id(), msg.location);
        } else {
            warn!("Attempted to spawn item at occupied position: {:?}", msg.location);
        }
    }
}

pub(super) fn despawn_terrain(
    mut commands: Commands,
    mut terrain_spatial_index: ResMut<SpatialIndex<Terrain>>,
    despawn_q: Query<(Entity, &Position), (With<Terrain>, With<Despawn>)>,
) {
    for (ent, pos) in &despawn_q {
        commands.entity(ent).despawn();
        terrain_spatial_index.remove(pos.0);
    }
}

pub(super) fn despawn_machine(
    mut commands: Commands,
    mut machine_spatial_index: ResMut<SpatialIndex<Machine>>,
    despawn_q: Query<(Entity, &Position), (With<Machine>, With<Despawn>)>,
) {
    for (ent, pos) in &despawn_q {
        commands.entity(ent).despawn();
        machine_spatial_index.remove(pos.0);
    }
}

pub(super) fn despawn_item(
    mut commands: Commands,
    mut item_spatial_index: ResMut<SpatialIndex<Item>>,
    despawn_q: Query<(Entity, &Position), (With<Item>, With<Despawn>)>,
) {
    for (ent, pos) in &despawn_q {
        commands.entity(ent).despawn();
        item_spatial_index.remove(pos.0);
    }
}
