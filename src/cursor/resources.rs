use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Cursor(pub Option<IVec2>);
