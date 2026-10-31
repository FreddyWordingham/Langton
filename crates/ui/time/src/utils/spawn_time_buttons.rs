use arc_theme::{DEFAULT_BUTTON_BORDER, DEFAULT_BUTTON_BORDER_COLOR, DEFAULT_BUTTON_BORDER_RADIUS, DEFAULT_BUTTON_PADDING};
use arc_tick::TickRate;
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::components::TimeButton;

pub(crate) fn spawn_time_buttons(parent: &mut ChildSpawnerCommands<'_>, assets: &AssetServer, is_paused: bool) {
    let mut spawn = |button| {
        parent
            .spawn(make_time_button(assets, button))
            .observe(move |_: On<Activate>, mut tick_rate: ResMut<TickRate>| {
                button.action(&mut tick_rate);
            });
    };

    if is_paused {
        spawn(TimeButton::Step);
    } else {
        spawn(TimeButton::Pause);
    }

    spawn(TimeButton::Speed1);
    spawn(TimeButton::Speed2);
    spawn(TimeButton::Speed3);
    spawn(TimeButton::Speed4);
}

fn make_time_button(assets: &AssetServer, time_button: TimeButton) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        Node {
            width: px(32.0),
            height: px(32.0),
            border: DEFAULT_BUTTON_BORDER,
            border_radius: DEFAULT_BUTTON_BORDER_RADIUS,
            padding: DEFAULT_BUTTON_PADDING,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderColor::from(DEFAULT_BUTTON_BORDER_COLOR),
        children![ImageNode::new(assets.load(time_button.icon_path()))],
    )
}
