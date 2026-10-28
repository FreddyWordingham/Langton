mod components;
mod constants;
mod plugin;
mod resources;
mod sets;
mod systems;

pub(crate) use components::Position;
pub(crate) use constants::TILE_SIZE;
pub(crate) use plugin::SpatialPlugin;
pub(crate) use resources::SpatialIndex;
pub(crate) use sets::SpatialUpdateSet;
use systems::sync_transforms;
