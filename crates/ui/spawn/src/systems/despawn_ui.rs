use bevy::prelude::*;

use crate::components::SpawnUi;

pub(crate) fn despawn_ui(mut commands: Commands, spawn_ui_q: Query<Entity, With<SpawnUi>>) {
    for ent in &spawn_ui_q {
        commands.entity(ent).despawn();
    }
}
