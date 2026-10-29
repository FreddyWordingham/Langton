use bevy::prelude::*;
use langton::prelude::*;

fn main() {
    App::new().add_plugins((DefaultPlugins, LangtonPlugin)).run();
}
