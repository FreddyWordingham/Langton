use arc_grid::Grid;
use arc_terrain::Terrain;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    terrain: Grid<Terrain>,
}

impl Map {
    pub fn new(size: UVec2) -> Self {
        Self {
            terrain: Grid::new(size, Terrain::Grass),
        }
    }

    pub fn terrain(&self) -> &Grid<Terrain> {
        &self.terrain
    }
}
