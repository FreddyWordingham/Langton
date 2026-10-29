use arc_tick::TickRate;
use bevy::prelude::*;

use crate::{
    components::{TimeMenu, TimeUi},
    utils::spawn_time_buttons,
};

pub(crate) fn spawn_ui(mut commands: Commands, assets: Res<AssetServer>, tick_rate: Res<TickRate>) {
    commands
        .spawn((
            TimeUi,
            Node {
                position_type: PositionType::Absolute,
                top: px(10.0),
                width: percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    TimeMenu(tick_rate.is_paused()),
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: px(5.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    spawn_time_buttons(parent, &assets, tick_rate.is_paused());
                });
        });
}
