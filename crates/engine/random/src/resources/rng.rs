use bevy::prelude::*;
use rand::rngs::SmallRng;

#[derive(Resource)]
pub struct Rng(pub SmallRng);
