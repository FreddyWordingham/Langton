use bevy::prelude::*;

use crate::time::TimeSpeed;

#[derive(Component)]
pub(super) struct TimeMenu;

#[derive(Component)]
pub(super) struct TimeSpeedButton(pub(super) TimeSpeed);
