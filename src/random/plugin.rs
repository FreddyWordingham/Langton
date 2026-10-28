use bevy::prelude::*;

use crate::random::Rng;

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rng(rand::make_rng()));
    }
}
