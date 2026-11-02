use bevy::prelude::*;

use crate::{messages::DestroyMap, resources::Map};

pub(crate) fn handle_destroy_map(
    mut commands: Commands,
    mut destroy_map_msg: MessageReader<DestroyMap>,
    map: Option<Res<Map>>,
) {
    let mut map_exists = map.is_some();

    for _ in destroy_map_msg.read() {
        if !map_exists {
            warn!("No map exists or destruction is already queued.");
            continue;
        }

        commands.remove_resource::<Map>();
        map_exists = false;
    }
}
