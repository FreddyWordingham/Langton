use arc_tick::{BASE_TPS, TickRate};
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub(crate) enum TimeButton {
    Pause,
    Step,
    Speed1,
    Speed2,
    Speed3,
    Speed4,
}

impl TimeButton {
    pub(crate) fn icon_path(&self) -> &'static str {
        match self {
            TimeButton::Pause => "sprites/icons/time/pause.png",
            TimeButton::Step => "sprites/icons/time/step.png",
            TimeButton::Speed1 => "sprites/icons/time/speed_1.png",
            TimeButton::Speed2 => "sprites/icons/time/speed_2.png",
            TimeButton::Speed3 => "sprites/icons/time/speed_3.png",
            TimeButton::Speed4 => "sprites/icons/time/speed_4.png",
        }
    }

    pub(crate) fn action(&self, tick_rate: &mut TickRate) {
        match self {
            TimeButton::Pause => tick_rate.pause(),
            TimeButton::Step => tick_rate.increment_pending(),
            TimeButton::Speed1 => {
                tick_rate.set_tps(BASE_TPS);
                tick_rate.unpause();
            }
            TimeButton::Speed2 => {
                tick_rate.set_tps(BASE_TPS * 2.0);
                tick_rate.unpause();
            }
            TimeButton::Speed3 => {
                tick_rate.set_tps(BASE_TPS * 4.0);
                tick_rate.unpause();
            }
            TimeButton::Speed4 => {
                tick_rate.set_tps(BASE_TPS * 8.0);
                tick_rate.unpause();
            }
        }
    }
}
