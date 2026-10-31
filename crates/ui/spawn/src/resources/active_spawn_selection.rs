use arc_construct::Construct;
use arc_feature::Feature;
use arc_item::Item;
use arc_terrain::Terrain;
use bevy::prelude::*;

#[derive(Resource)]
pub enum ActiveSpawnSelection {
    Terrain(Terrain),
    Feature(Feature),
    Construct(Construct),
    Item(Item),
}
