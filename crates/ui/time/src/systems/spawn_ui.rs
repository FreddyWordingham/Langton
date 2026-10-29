use bevy::prelude::*;

use crate::components::TimeUi;

pub(crate) fn spawn_ui(mut commands: Commands) {
    commands
        .spawn((
            TimeUi,
            Node {
                position_type: PositionType::Absolute,
                top: px(0.0),
                width: percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    height: px(50.0),
                    width: px(300.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
            ));
        });
}
