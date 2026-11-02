use bevy::prelude::*;

pub fn iter_coords(size: UVec2) -> impl Iterator<Item = UVec2> {
    (0..size.y).flat_map(move |y| (0..size.x).map(move |x| UVec2::new(x, y)))
}
