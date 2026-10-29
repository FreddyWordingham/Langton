use bevy::prelude::*;

use crate::{
    components::MainCamera,
    constants::{MAX_ZOOM, MIN_ZOOM, ZOOM_SPEED},
    resources::CameraControl,
};

pub(crate) fn zoom_camera(
    camera_control: Res<CameraControl>,
    time: Res<Time<Real>>,
    mut main_camera_q: Query<&mut Projection, With<MainCamera>>,
) {
    for mut camera_projection in &mut main_camera_q {
        let Projection::Orthographic(ref mut orthographic) = *camera_projection else {
            continue;
        };

        let factor = (-camera_control.zoom * ZOOM_SPEED * time.delta_secs()).exp();

        orthographic.scale = (orthographic.scale * factor).clamp(MIN_ZOOM, MAX_ZOOM);
    }
}
