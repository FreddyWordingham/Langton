use bevy::prelude::*;

use crate::editor::{EditorSelection, EditorTab};

#[derive(Resource, Default)]
pub(super) struct ShowEditorUi;

#[derive(Resource)]
pub(super) struct ActiveEditorTab(pub(super) EditorTab);

#[derive(Resource)]
pub(crate) struct ActiveEditorSelection(pub(crate) EditorSelection);
