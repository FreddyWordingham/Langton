use arc_gizmo::GizmoPlugin;
use bevy::prelude::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GizmoPlugin);
    }
}
