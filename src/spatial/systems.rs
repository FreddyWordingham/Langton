use bevy::prelude::*;

use crate::spatial::{Position, TILE_SIZE};

pub(super) fn sync_transforms(mut query: Query<(&mut Transform, &Position), Changed<Position>>) {
    for (mut transform, position) in &mut query {
        transform.translation = (position.0.as_vec2() * TILE_SIZE).extend(transform.translation.z);
    }
}
