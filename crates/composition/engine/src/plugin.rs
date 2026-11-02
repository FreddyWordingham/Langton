use arc_map::{CreateMap, Map, MapPlugin};
use arc_spatial::SpatialPlugin;
use arc_tick::TickPlugin;
use bevy::prelude::*;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MapPlugin, SpatialPlugin, TickPlugin))
            .add_systems(Startup, setup);
    }
}

fn setup(mut create_map_msg: MessageWriter<CreateMap>) {
    create_map_msg.write(CreateMap {
        map: Map::new(uvec2(32, 32)),
    });
}
