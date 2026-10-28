mod plugin;
mod sets;
mod systems;
mod traits;

pub(crate) use plugin::RenderPlugin;
pub(crate) use sets::RenderUpdateSet;
use systems::sync_appearances;
pub(crate) use traits::Appearance;
