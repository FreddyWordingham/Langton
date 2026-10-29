use bevy::prelude::*;

use crate::resources::TickRate;

pub(crate) fn step_tick(
    mut tick_rate: ResMut<TickRate>,
    mut virtual_time: ResMut<Time<Virtual>>,
    fixed_time: Res<Time<Fixed>>,
    mut time: ResMut<Time>,
) {
    if !tick_rate.is_paused() {
        return;
    }

    if tick_rate.decrement_pending() {
        virtual_time.advance_by(fixed_time.timestep());
        *time = virtual_time.as_generic();
    }
}
