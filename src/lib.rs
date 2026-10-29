use bevy::prelude::*;

mod camera;
mod cursor;
mod direction;
mod editor;
mod gas;
mod lifecycle;
mod pumpjack;
mod random;
mod render;
mod spatial;
mod thing;
mod ui;

use crate::{
    camera::{CameraPlugin, CameraStartupSet, CameraUpdateSet},
    cursor::{CursorPlugin, CursorPostUpdateSet},
    editor::{EditorPlugin, EditorUpdateSet},
    gas::{GasFixedUpdateSet, GasPlugin},
    lifecycle::{LifecycleFixedUpdateSet, LifecyclePlugin},
    pumpjack::{PumpjackFixedUpdateSet, PumpjackPlugin},
    random::RandomPlugin,
    render::{RenderPlugin, RenderUpdateSet},
    spatial::{SpatialPlugin, SpatialUpdateSet},
    ui::{UiPlugin, UiUpdateSet},
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
            EditorPlugin,
            GasPlugin,
            LifecyclePlugin,
            PumpjackPlugin,
            RandomPlugin,
            RenderPlugin,
            SpatialPlugin,
            UiPlugin,
        ))
        .configure_sets(Startup, CameraStartupSet::Spawn)
        .configure_sets(
            Update,
            (
                UiUpdateSet::Sync,
                EditorUpdateSet::Input,
                EditorUpdateSet::Spawn,
                EditorUpdateSet::Despawn,
                CameraUpdateSet::Input,
                RenderUpdateSet::Sync,
                SpatialUpdateSet::Sync,
            ),
        )
        .configure_sets(PostUpdate, CursorPostUpdateSet::Sync)
        .configure_sets(
            FixedUpdate,
            (
                LifecycleFixedUpdateSet::Spawn,
                PumpjackFixedUpdateSet::Create,
                GasFixedUpdateSet::Move,
                LifecycleFixedUpdateSet::Despawn,
            ),
        );
    }
}
