use bevy::prelude::*;

use crate::time::DEFAULT_TPS;

#[derive(Resource)]
pub(super) struct TickRate(pub(super) Option<f64>);

#[derive(Resource, Default)]
pub(super) struct PendingSteps(pub(super) u32);

impl Default for TickRate {
    fn default() -> Self {
        Self(DEFAULT_TPS)
    }
}
