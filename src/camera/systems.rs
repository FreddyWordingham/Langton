use bevy::prelude::*;

use crate::camera::{
    MainCamera, PAN_EAST_KEY, PAN_NORTH_KEY, PAN_SOUTH_KEY, PAN_SPEED, PAN_WEST_KEY, ZOOM_IN_KEY, ZOOM_OUT_KEY, ZOOM_SPEED,
};

pub(super) fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d));
}

pub(super) fn pan_main_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(PAN_NORTH_KEY) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(PAN_SOUTH_KEY) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(PAN_WEST_KEY) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(PAN_EAST_KEY) {
            direction.x += 1.0;
        }
        transform.translation += direction * PAN_SPEED * time.delta_secs();
    }
}

pub(super) fn zoom_main_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        if keyboard_input.pressed(ZOOM_IN_KEY) {
            transform.scale -= Vec3::splat(ZOOM_SPEED * time.delta_secs());
        }
        if keyboard_input.pressed(ZOOM_OUT_KEY) {
            transform.scale += Vec3::splat(ZOOM_SPEED * time.delta_secs());
        }
    }
}
