use arc_construct::ConstructPlugin;
use arc_feature::FeaturePlugin;
use arc_item::ItemPlugin;
use arc_terrain::TerrainPlugin;
use bevy::prelude::*;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ConstructPlugin, FeaturePlugin, ItemPlugin, TerrainPlugin));
    }
}
