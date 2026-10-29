mod constants;
mod plugin;
mod sets;
mod systems;

use constants::{HOVERED_COLOR, IDLE_COLOR, PRESSED_COLOR};
pub(crate) use plugin::UiPlugin;
pub(crate) use sets::UiUpdateSet;
use systems::update_button_colors;
