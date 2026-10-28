use bevy::prelude::*;

use crate::{
    gas::Gas,
    pumpjack::{PUMPJACK_COOLDOWN, Pumpjack},
    spatial::{Position, SpatialIndex},
    thing::{Item, Terrain},
};

pub(super) fn create_items(
    mut commands: Commands,
    mut item_spatial_index: ResMut<SpatialIndex<Item>>,
    mut pumpjack_q: Query<(&mut Pumpjack, &Position)>,
    terrain_q: Query<&Terrain>,
) {
    for (mut pumpjack, pos) in &mut pumpjack_q {
        if pumpjack.remaining_cooldown > 0 {
            pumpjack.remaining_cooldown = pumpjack.remaining_cooldown.saturating_sub(1);
            continue;
        }

        if let Ok(terrain) = terrain_q.get(pumpjack.target) {
            if item_spatial_index.get(&pos.0).is_none() {
                let item = terrain.item();
                let mut ent = commands.spawn((Position(pos.0), item));

                if terrain.item().is_gas() {
                    ent.insert(Gas);
                }

                item_spatial_index.insert(ent.id(), pos.0);
                pumpjack.remaining_cooldown = PUMPJACK_COOLDOWN;
            }
        }
    }
}
