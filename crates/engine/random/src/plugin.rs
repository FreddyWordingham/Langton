use bevy::prelude::*;

use crate::resources::Rng;

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rng(rand::make_rng()));
    }
}
