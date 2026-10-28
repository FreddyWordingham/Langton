use bevy::{camera::CameraUpdateSystems, prelude::*, transform::TransformSystems};

use crate::cursor::{Cursor, CursorSet, sync_cursor};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PostUpdate, CursorSet::Sync)
            .init_resource::<Cursor>()
            .add_systems(
                PostUpdate,
                sync_cursor.after(TransformSystems::Propagate).after(CameraUpdateSystems),
            )
            .add_systems(Update, report_cursor.run_if(resource_changed::<Cursor>));
    }
}

fn report_cursor(cursor: Res<Cursor>) {
    println!("Cursor position: {:?}", cursor.0);
}
