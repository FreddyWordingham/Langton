use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Map {
    terrain: Grid<Terrain>,
}

impl Map {
    pub fn new(&self, size: UVec2) -> u64 {}
}
