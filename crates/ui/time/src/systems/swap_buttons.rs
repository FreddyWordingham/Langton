use arc_tick::TickRate;
use bevy::prelude::*;

use crate::{components::TimeMenu, utils::spawn_time_buttons};

pub(crate) fn swap_buttons(
    mut commands: Commands,
    assets: Res<AssetServer>,
    tick_rate: Res<TickRate>,
    mut time_menu_q: Query<(Entity, &mut TimeMenu)>,
) {
    for (ent, mut time_menu) in &mut time_menu_q {
        if time_menu.0 == tick_rate.is_paused() {
            continue;
        }

        time_menu.0 = tick_rate.is_paused();
        commands.entity(ent).despawn_children().with_children(|parent| {
            spawn_time_buttons(parent, &assets, tick_rate.is_paused());
        });
    }
}
