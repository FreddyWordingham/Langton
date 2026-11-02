use bevy::prelude::*;

use crate::{messages::CreateMap, resources::Map};

pub(crate) fn handle_create_map(
    mut commands: Commands,
    mut create_map_msg: ResMut<Messages<CreateMap>>,
    map: Option<Res<Map>>,
) {
    let mut map_exists = map.is_some();

    for msg in create_map_msg.drain() {
        if map_exists {
            warn!("Map already exists or is queued for creation.");
            continue;
        }

        commands.insert_resource(msg.map);
        map_exists = true;
    }
}
