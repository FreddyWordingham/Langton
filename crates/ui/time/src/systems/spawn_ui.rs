use arc_tick::TickRate;
use bevy::{picking::hover::Hovered, prelude::*, ui_widgets::Button};

use crate::components::{TimeButton, TimeMenu, TimeUi};

pub(crate) fn spawn_ui(mut commands: Commands, assets: Res<AssetServer>, tick_rate: Res<TickRate>) {
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
                TimeMenu,
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(5.0),
                    ..default()
                },
            ));

            if tick_rate.is_paused() {
                parent.spawn(make_time_button(&assets, TimeButton::Play));
                parent.spawn(make_time_button(&assets, TimeButton::Step));
            } else {
                parent.spawn(make_time_button(&assets, TimeButton::Pause));
                parent.spawn(make_time_button(&assets, TimeButton::Play));
                parent.spawn(make_time_button(&assets, TimeButton::FastForward));
            }
        });
}

fn make_time_button(assets: &Res<AssetServer>, time_button: TimeButton) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        Node {
            width: px(32.0),
            height: px(32.0),
            ..default()
        },
        ImageNode::new(assets.load(time_button.icon_path())),
    )
}
