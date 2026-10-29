use bevy::prelude::*;

use crate::{components::MainCamera, constants::CLEAR_COLOR};

pub(crate) fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(CLEAR_COLOR),
            ..default()
        },
    ));
}
