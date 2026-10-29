mod components;
mod constants;
mod plugin;
mod resources;
mod sets;
mod systems;
mod types;

use components::{EditorMenu, EditorSelectionButton, EditorSelectionPanel, EditorTabButton, EditorTabPanel};
use constants::TOGGLE_EDITOR_UI_KEY;
pub(crate) use plugin::EditorPlugin;
pub(crate) use resources::ActiveEditorSelection;
use resources::{ActiveEditorTab, ShowEditorUi};
pub(crate) use sets::EditorUpdateSet;
use systems::{
    despawn_menu, despawn_selection_panel, spawn_menu, spawn_selection_panel, sync_button_selection, toggle_editor_ui,
    trigger_spawn_thing,
};
use types::{EditorSelection, EditorTab};
