use bevy::{prelude::*, window::PrimaryWindow};

use crate::{camera::MainCamera, cursor::Cursor, spatial::TILE_SIZE};

pub(crate) fn sync_cursor(
    mut cursor: ResMut<Cursor>,
    window: Single<&Window, With<PrimaryWindow>>,
    main_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = *main_camera;
    let new_pos = if let Some(position) = window.cursor_position()
        && let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, position)
    {
        Some((world_position / TILE_SIZE + Vec2::splat(0.5)).floor().as_ivec2())
    } else {
        None
    };

    if cursor.0 != new_pos {
        cursor.0 = new_pos;
    }
}
