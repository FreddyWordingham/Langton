use bevy::prelude::*;

use crate::{components::MainCamera, constants::PAN_SPEED, resources::CameraControl};

pub(crate) fn pan_camera(
    camera_control: Res<CameraControl>,
    time: Res<Time<Real>>,
    mut main_camera_q: Query<&mut Transform, With<MainCamera>>,
) {
    for mut camera_transform in &mut main_camera_q {
        camera_transform.translation += (camera_control.pan * PAN_SPEED * time.delta_secs()).extend(0.0);
    }
}
