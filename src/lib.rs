use bevy::prelude::*;

mod camera;
mod cursor;
mod item;
mod lifecycle;
mod render;
mod spatial;

use crate::{
    camera::CameraPlugin, cursor::CursorPlugin, lifecycle::LifecyclePlugin, render::RenderPlugin, spatial::SpatialPlugin,
};

pub mod prelude {
    pub use crate::{LangtonPlugin, cursor::Cursor, item::Item, lifecycle::SpawnThing};
}

pub struct LangtonPlugin;

impl Plugin for LangtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, CursorPlugin, LifecyclePlugin, SpatialPlugin, RenderPlugin));
    }
}
