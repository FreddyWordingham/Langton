use bevy::{camera::CameraUpdateSystems, prelude::*, transform::TransformSystems};

use crate::cursor::{Cursor, CursorPostUpdateSet, sync_cursor};

pub(crate) struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            CursorPostUpdateSet::Sync
                .after(TransformSystems::Propagate)
                .after(CameraUpdateSystems),
        )
        .init_resource::<Cursor>()
        .add_systems(PostUpdate, sync_cursor.in_set(CursorPostUpdateSet::Sync));
    }
}
