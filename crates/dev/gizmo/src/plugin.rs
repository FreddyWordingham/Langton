use bevy::prelude::*;

use crate::{sets::GizmoUpdate, systems::draw_grid};

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GizmoUpdate::Draw)
            .add_systems(Update, draw_grid.in_set(GizmoUpdate::Draw));
    }
}
