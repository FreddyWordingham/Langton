use bevy::prelude::*;

use crate::constants::{BASE_TPS, START_PAUSED};

pub(crate) fn init_time(mut virtual_time: ResMut<Time<Virtual>>, mut fixed_time: ResMut<Time<Fixed>>) {
    fixed_time.set_timestep_hz(BASE_TPS);

    if START_PAUSED {
        virtual_time.pause();
    } else {
        virtual_time.unpause();
    }
}
