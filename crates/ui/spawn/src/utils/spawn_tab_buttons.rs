use arc_button::{button, button_node};
use bevy::{prelude::*, ui_widgets::Activate};

use crate::{resources::ShowSpawnUi, types::SpawnTab};

pub(crate) fn spawn_tab_buttons(parent: &mut ChildSpawnerCommands<'_>) {
    for tab in SpawnTab::ALL {
        parent
            .spawn((
                button(Node {
                    width: percent(100.0),
                    height: auto(),
                    ..button_node()
                }),
                children![Text::new(tab.as_str())],
            ))
            .observe(move |_: On<Activate>, mut commands: Commands| {
                commands.insert_resource_if_neq(ShowSpawnUi(tab));
            });
    }
}
