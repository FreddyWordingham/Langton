use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
#[require(Transform)]
pub(crate) struct Position(pub(crate) IVec2);
