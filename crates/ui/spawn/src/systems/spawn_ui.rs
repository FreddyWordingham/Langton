use bevy::prelude::*;

use crate::{
    components::{SpawnSelectionPanel, SpawnTabPanel, SpawnUi},
    utils::spawn_tab_buttons,
};

pub(crate) fn spawn_ui(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            SpawnUi,
            Node {
                position_type: PositionType::Absolute,
                width: px(200.0),
                height: percent(100.0),
                padding: UiRect::all(px(5.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SpawnTabPanel,
                    Node {
                        width: percent(100.0),
                        height: auto(),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(5.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    spawn_tab_buttons(parent, &assets);
                });

            parent.spawn((
                SpawnSelectionPanel,
                Node {
                    width: percent(100.0),
                    height: percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexEnd,
                    row_gap: px(5.0),
                    ..default()
                },
            ));
        });
}
