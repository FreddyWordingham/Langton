use bevy::prelude::*;

use crate::editor::{EditorSelection, EditorTab};

#[derive(Component)]
pub(super) struct EditorMenu;

#[derive(Component)]
pub(super) struct EditorTabPanel;

#[derive(Component)]
pub(super) struct EditorSelectionPanel;

#[derive(Component)]
pub(super) struct EditorTabButton(pub(super) EditorTab);

#[derive(Component)]
pub(super) struct EditorSelectionButton(pub(super) EditorSelection);
