use bevy::prelude::*;

use crate::resources::Tick;

pub(crate) fn increment_tick(mut tick: ResMut<Tick>) {
    tick.increment();
    info!("Tick {}", tick.tick());
}
