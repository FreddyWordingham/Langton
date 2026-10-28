use bevy::prelude::*;
use rand::RngExt;

use crate::{
    direction::Direction,
    gas::Gas,
    random::Rng,
    spatial::{Position, SpatialIndex},
    thing::Item,
};

pub(super) fn move_gas(
    mut item_spatial_index: ResMut<SpatialIndex<Item>>,
    mut rng: ResMut<Rng>,
    mut gas_q: Query<&mut Position, With<Gas>>,
) {
    for mut pos in &mut gas_q {
        let direction = match rng.0.random_range(0..4) {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!(),
        };
        let new_pos = pos.0 + direction.offset();

        if item_spatial_index.get(&new_pos).is_none() {
            item_spatial_index.transfer(pos.0, new_pos);
            pos.0 = new_pos;
        }
    }
}
