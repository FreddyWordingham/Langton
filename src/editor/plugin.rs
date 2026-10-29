use bevy::{
    picking::{hover::HoverMap, pointer::PointerId},
    prelude::*,
};

use crate::editor::{
    ActiveEditorSelection, ActiveEditorTab, EditorUpdateSet, ShowEditorUi, despawn_menu, despawn_selection_panel, spawn_menu,
    spawn_selection_panel, sync_button_selection, toggle_editor_ui, trigger_spawn_thing,
};

pub(crate) struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                EditorUpdateSet::Spawn,
                EditorUpdateSet::Input,
                EditorUpdateSet::Despawn,
                EditorUpdateSet::Sync,
            )
                .chain(),
        )
        .init_resource::<ShowEditorUi>()
        .add_systems(
            Update,
            (
                (
                    spawn_menu.run_if(resource_added::<ShowEditorUi>),
                    spawn_selection_panel.run_if(resource_exists_and_changed::<ActiveEditorTab>),
                )
                    .in_set(EditorUpdateSet::Spawn),
                (
                    toggle_editor_ui,
                    trigger_spawn_thing.run_if(resource_exists::<ActiveEditorSelection>.and_eager(pointer_not_over_ui)),
                )
                    .in_set(EditorUpdateSet::Input),
                (
                    despawn_menu.run_if(resource_removed::<ShowEditorUi>),
                    despawn_selection_panel.run_if(resource_removed::<ActiveEditorTab>),
                )
                    .in_set(EditorUpdateSet::Despawn),
                sync_button_selection.in_set(EditorUpdateSet::Sync),
            ),
        );
    }
}

fn pointer_not_over_ui(hover_map: Res<HoverMap>, ui_nodes: Query<(), With<Node>>) -> bool {
    !hover_map
        .get(&PointerId::Mouse)
        .is_some_and(|hits| hits.keys().any(|entity| ui_nodes.contains(*entity)))
}
