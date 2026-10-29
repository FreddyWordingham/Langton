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
    time: Res<Time<Real>>,
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
    time: Res<Time<Real>>,
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::time::TimeUpdateStrategy;

    use super::*;

    #[test]
    fn camera_moves_and_zooms_while_virtual_time_is_paused() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(50)))
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(Update, (pan_main_camera, zoom_main_camera));
        let camera = app.world_mut().spawn((MainCamera, Transform::default())).id();
        app.world_mut().resource_mut::<Time<Virtual>>().pause();
        {
            let mut keyboard = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keyboard.press(PAN_EAST_KEY);
            keyboard.press(ZOOM_IN_KEY);
        }
        app.update();
        app.update();

        let transform = app.world().get::<Transform>(camera).unwrap();
        assert!((transform.translation.x - PAN_SPEED * 0.05).abs() < 0.0001);
        assert!((transform.scale.x - (1.0 - ZOOM_SPEED * 0.05)).abs() < 0.0001);
        assert_eq!(app.world().resource::<Time<Virtual>>().delta(), Duration::ZERO);
    }
}
