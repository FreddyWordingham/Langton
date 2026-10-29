use bevy::prelude::*;

use crate::ui::{UiUpdateSet, update_button_colors};

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, UiUpdateSet::Sync)
            .add_systems(Update, update_button_colors.in_set(UiUpdateSet::Sync));
    }
}
