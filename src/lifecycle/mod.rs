mod components;
mod messages;
mod plugin;
mod sets;
mod systems;

pub(crate) use components::Despawn;
pub use messages::{SpawnItem, SpawnMachine, SpawnTerrain};
pub(crate) use plugin::LifecyclePlugin;
pub(crate) use sets::LifecycleFixedUpdateSet;
use systems::{despawn_item, despawn_machine, despawn_terrain, handle_spawn_item, handle_spawn_machine, handle_spawn_terrain};
