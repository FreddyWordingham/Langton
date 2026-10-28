mod components;
mod plugin;
mod sets;
mod systems;

pub(crate) use components::Colored;
pub(crate) use plugin::RenderPlugin;
pub(crate) use sets::RenderSet;
use systems::sync_meshes;
