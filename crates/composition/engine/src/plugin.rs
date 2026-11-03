use arc_map::{CreateMap, Map, MapPlugin};
use arc_random::RandomPlugin;
use arc_spatial::SpatialPlugin;
use arc_tick::TickPlugin;
use bevy::prelude::*;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MapPlugin, RandomPlugin, SpatialPlugin, TickPlugin))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, mutate_map.run_if(resource_exists::<Map>));
    }
}

fn setup(mut create_map_msg: MessageWriter<CreateMap>) {
    create_map_msg.write(CreateMap {
        map: Map::new(uvec2(32, 32)),
    });
}

fn mutate_map(mut map: ResMut<Map>, mut rng: ResMut<arc_random::Rng>) {
    use rand::RngExt;

    let size = map.terrain().size();
    let coord = uvec2(rng.0.random_range(0..size.x), rng.0.random_range(0..size.y));
    map.terrain_mut()[coord] = Some(arc_terrain::Terrain::Sand);
}
