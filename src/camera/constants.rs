use bevy::prelude::*;

pub(super) const CLEAR_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

pub(super) const PAN_SPEED: f32 = 500.0;
pub(super) const ZOOM_SPEED: f32 = 1.0;

pub(super) const PAN_NORTH_KEY: KeyCode = KeyCode::KeyW;
pub(super) const PAN_SOUTH_KEY: KeyCode = KeyCode::KeyS;
pub(super) const PAN_WEST_KEY: KeyCode = KeyCode::KeyA;
pub(super) const PAN_EAST_KEY: KeyCode = KeyCode::KeyD;
pub(super) const ZOOM_IN_KEY: KeyCode = KeyCode::Equal;
pub(super) const ZOOM_OUT_KEY: KeyCode = KeyCode::Minus;
