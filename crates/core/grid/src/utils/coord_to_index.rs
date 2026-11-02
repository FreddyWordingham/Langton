use bevy::prelude::*;

pub fn coord_to_index(coord: UVec2, size: UVec2) -> usize {
    assert!(
        coord.x < size.x && coord.y < size.y,
        "Coordinate out of bounds: the coordinate is {:?} but the grid size is {:?}",
        coord,
        size
    );
    (coord.y as usize)
        .checked_mul(size.x as usize)
        .and_then(|row_start| row_start.checked_add(coord.x as usize))
        .expect("Grid index cannot fit in usize")
}
