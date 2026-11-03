use arc_grid::DirtyTrackingGrid;
use arc_terrain::Terrain;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Map {
    terrain: DirtyTrackingGrid<Option<Terrain>>,
}

impl Map {
    pub fn new(size: UVec2) -> Self {
        Self {
            terrain: DirtyTrackingGrid::new(size, Some(Terrain::Soil)),
        }
    }

    pub fn terrain(&self) -> &DirtyTrackingGrid<Option<Terrain>> {
        &self.terrain
    }

    pub fn terrain_mut(&mut self) -> &mut DirtyTrackingGrid<Option<Terrain>> {
        &mut self.terrain
    }
}
