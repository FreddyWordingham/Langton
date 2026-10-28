use bevy::prelude::*;

mod item;
mod lifecycle;
mod spatial;

use crate::{lifecycle::LifecyclePlugin, spatial::SpatialPlugin};

pub mod prelude {
    pub use crate::LangtonPlugin;
}

pub struct LangtonPlugin;

impl Plugin for LangtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LifecyclePlugin, SpatialPlugin));
    }
}
