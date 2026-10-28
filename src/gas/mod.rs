mod components;
mod plugin;
mod sets;
mod systems;

pub(crate) use components::Gas;
pub(crate) use plugin::GasPlugin;
pub(crate) use sets::GasFixedUpdateSet;
use systems::move_gas;
