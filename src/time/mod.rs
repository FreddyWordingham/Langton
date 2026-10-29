mod components;
mod constants;
mod plugin;
mod resources;
mod sets;
mod systems;
mod types;

use components::{TimeMenu, TimeSpeedButton};
use constants::DEFAULT_TPS;
pub(crate) use plugin::TimePlugin;
use resources::{PendingSteps, TickRate};
pub(crate) use sets::{TimeStartupSet, TimeUpdateSet};
use systems::{spawn_menu, step_time, update_tick_rate};
use types::TimeSpeed;
