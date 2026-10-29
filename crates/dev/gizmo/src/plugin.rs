use arc_core::TILE_SIZE;
use bevy::prelude::*;

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_grid);
    }
}

fn draw_grid(mut gizmos: Gizmos) {
    gizmos.rect_2d(Isometry2d::default(), Vec2::splat(TILE_SIZE), Color::srgb(1.0, 0.0, 0.0));
    info!("Drawing grid");
}
