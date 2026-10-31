use bevy::prelude::*;

use crate::{components::SpawnSelectionPanel, resources::ShowSpawnUi, utils::spawn_selection_buttons};

pub(crate) fn update_spawn_selection_buttons(
    mut commands: Commands,
    assets: Res<AssetServer>,
    show_spawn_ui: Res<ShowSpawnUi>,
    spawn_selection_panel_q: Query<Entity, With<SpawnSelectionPanel>>,
) {
    for ent in &spawn_selection_panel_q {
        commands.entity(ent).despawn_children().with_children(|parent| {
            spawn_selection_buttons(parent, &assets, show_spawn_ui.0);
        });
    }
}
