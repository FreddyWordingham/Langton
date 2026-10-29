use bevy::prelude::*;

use crate::resources::TickRate;

pub(crate) fn update_tps(
    tick_rate: Res<TickRate>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut fixed_time: ResMut<Time<Fixed>>,
) {
    if let Some(tps) = tick_rate.tps() {
        fixed_time.set_timestep_hz(tps);
        virtual_time.unpause();
    } else {
        virtual_time.pause();
    };
}
