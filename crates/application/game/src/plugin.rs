use arc_content::ContentPlugin;
#[cfg(feature = "dev")]
use arc_dev::DevPlugin;
use arc_engine::EnginePlugin;
use arc_input::InputPlugin;
use arc_presentation::PresentationPlugin;
use arc_simulation::SimulationPlugin;
use arc_ui::UiPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ContentPlugin,
            EnginePlugin,
            InputPlugin,
            PresentationPlugin,
            SimulationPlugin,
            UiPlugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(DevPlugin);
    }
}
