use bevy::prelude::*;

use crate::types::SpawnTab;

#[derive(Resource, PartialEq, Eq)]
pub struct ShowSpawnUi(pub(crate) SpawnTab);

impl ShowSpawnUi {
    pub fn terrain() -> Self {
        Self(SpawnTab::Terrain)
    }

    pub fn feature() -> Self {
        Self(SpawnTab::Feature)
    }

    pub fn construct() -> Self {
        Self(SpawnTab::Construct)
    }

    pub fn item() -> Self {
        Self(SpawnTab::Item)
    }
}
