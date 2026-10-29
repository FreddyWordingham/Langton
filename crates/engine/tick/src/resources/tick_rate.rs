use bevy::prelude::*;

use crate::constants::{DEFAULT_TPS, START_PAUSED};

#[derive(Resource)]
pub struct TickRate {
    is_paused: bool,
    tps: f64,
    pending_steps: u32,
}

impl Default for TickRate {
    fn default() -> Self {
        Self {
            is_paused: START_PAUSED,
            tps: DEFAULT_TPS,
            pending_steps: 0,
        }
    }
}

impl TickRate {
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub(crate) fn tps(&self) -> Option<f64> {
        if self.is_paused { None } else { Some(self.tps) }
    }

    pub fn set_tps(&mut self, tps: f64) {
        self.tps = tps;
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
        self.pending_steps = 0;
    }

    pub fn unpause(&mut self) {
        self.is_paused = false;
        self.pending_steps = 0;
    }

    pub fn increment_pending(&mut self) {
        self.pending_steps += 1;
    }

    pub(crate) fn decrement_pending(&mut self) -> bool {
        if self.pending_steps > 0 {
            self.pending_steps -= 1;
            true
        } else {
            false
        }
    }
}
