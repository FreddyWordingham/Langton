use bevy::prelude::*;

mod camera;
mod cursor;
mod direction;
mod lifecycle;
mod pumpjack;
mod random;
mod render;
mod spatial;
mod thing;

use crate::{
    camera::CameraPlugin, cursor::CursorPlugin, lifecycle::LifecyclePlugin, pumpjack::PumpjackPlugin, random::RandomPlugin,
    render::RenderPlugin, spatial::SpatialPlugin,
};

pub mod prelude {
    pub use crate::{
        LangtonPlugin,
        cursor::Cursor,
        lifecycle::{SpawnItem, SpawnMachine, SpawnTerrain},
        random::Rng,
        thing::{Item, Machine, Terrain},
    };
}

pub struct LangtonPlugin;

impl Plugin for LangtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CameraPlugin,
            CursorPlugin,
            LifecyclePlugin,
            PumpjackPlugin,
            RandomPlugin,
            RenderPlugin,
            SpatialPlugin,
        ));
    }
}
