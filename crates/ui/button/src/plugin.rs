use bevy::prelude::*;

use crate::{sets::ButtonUpdate, systems::update_button_colors};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, ButtonUpdate::Sync)
            .add_systems(Update, update_button_colors.in_set(ButtonUpdate::Sync));
    }
}
