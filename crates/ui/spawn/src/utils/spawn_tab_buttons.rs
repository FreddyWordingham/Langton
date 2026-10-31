use arc_theme::{DEFAULT_BUTTON_BORDER, DEFAULT_BUTTON_BORDER_COLOR, DEFAULT_BUTTON_BORDER_RADIUS, DEFAULT_BUTTON_PADDING};
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::{resources::ShowSpawnUi, types::SpawnTab};

pub(crate) fn spawn_tab_buttons(parent: &mut ChildSpawnerCommands<'_>) {
    for tab in SpawnTab::ALL {
        parent
            .spawn((
                Button,
                Hovered::default(),
                Node {
                    width: percent(100.0),
                    height: auto(),
                    border: DEFAULT_BUTTON_BORDER,
                    border_radius: DEFAULT_BUTTON_BORDER_RADIUS,
                    padding: DEFAULT_BUTTON_PADDING,
                    ..default()
                },
                BorderColor::from(DEFAULT_BUTTON_BORDER_COLOR),
                children![Text::new(tab.as_str())],
            ))
            .observe(move |_: On<Activate>, mut commands: Commands| {
                commands.insert_resource_if_neq(ShowSpawnUi(tab));
            });
    }
}
