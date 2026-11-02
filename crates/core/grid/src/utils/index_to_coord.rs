use bevy::prelude::*;

pub fn index_to_coord(index: usize, size: UVec2) -> UVec2 {
    let len = (size.x as usize)
        .checked_mul(size.y as usize)
        .expect("Grid length cannot fit in usize");
    assert!(
        index < len,
        "Index out of bounds: the index is {} but the grid length is {}",
        index,
        len
    );
    let x = (index % size.x as usize) as u32;
    let y = (index / size.x as usize) as u32;
    UVec2::new(x, y)
}
