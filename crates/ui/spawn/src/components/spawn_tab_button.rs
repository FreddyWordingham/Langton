use bevy::prelude::*;

use crate::types::SpawnTab;

#[derive(Component)]
pub(crate) struct SpawnTabButton(pub(crate) SpawnTab);
