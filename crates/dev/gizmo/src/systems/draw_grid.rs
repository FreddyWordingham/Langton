use arc_spatial::TILE_SIZE;
use bevy::prelude::*;

pub(crate) fn draw_grid(mut gizmos: Gizmos) {
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::new(16, 16),
            Vec2::splat(TILE_SIZE),
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}
