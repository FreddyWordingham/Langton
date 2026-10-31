use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::{components::SpawnTabButton, resources::ShowSpawnUi, types::SpawnTab};

pub(crate) fn spawn_tab_buttons(parent: &mut ChildSpawnerCommands<'_>, assets: &AssetServer) {
    for tab in SpawnTab::ALL {
        parent
            .spawn((
                SpawnTabButton(tab),
                Button,
                Hovered::default(),
                Node {
                    width: percent(100.0),
                    height: auto(),
                    border: UiRect::all(px(2.0)),
                    border_radius: BorderRadius::all(px(4.0)),
                    padding: UiRect::left(px(5.0)),
                    ..default()
                },
                BorderColor::from(Color::srgb(0.4, 0.4, 0.4)),
                children![Text::new(tab.as_str())],
            ))
            .observe(move |_: On<Activate>, mut commands: Commands| {
                commands.insert_resource(ShowSpawnUi(tab));
            });
    }
}
