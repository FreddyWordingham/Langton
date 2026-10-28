mod components;
mod messages;
mod plugin;
mod sets;
mod systems;
mod traits;

pub(crate) use components::Despawn;
pub(crate) use messages::SpawnThing;
pub(crate) use plugin::LifecyclePlugin;
pub(crate) use sets::LifecycleSet;
use systems::{despawn_marked, handle_spawn_thing};
pub(crate) use traits::ApplyTo;
