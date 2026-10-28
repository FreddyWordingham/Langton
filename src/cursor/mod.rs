mod plugin;
mod resources;
mod sets;
mod systems;

pub(crate) use plugin::CursorPlugin;
pub use resources::Cursor;
pub(crate) use sets::CursorPostUpdateSet;
use systems::sync_cursor;
