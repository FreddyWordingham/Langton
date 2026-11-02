use bevy::prelude::*;

use crate::{
    messages::{CreateMap, DestroyMap},
    sets::MapFixedUpdate,
    systems::{handle_create_map, handle_destroy_map},
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, (MapFixedUpdate::Create, MapFixedUpdate::Destroy).chain())
            .add_message::<CreateMap>()
            .add_message::<DestroyMap>()
            .add_systems(
                FixedUpdate,
                (
                    handle_create_map.in_set(MapFixedUpdate::Create),
                    handle_destroy_map.in_set(MapFixedUpdate::Destroy),
                ),
            );
    }
}
