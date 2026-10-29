use arc_tick::TickRate;
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::components::TimeButton;

pub(crate) fn spawn_time_buttons(parent: &mut ChildSpawnerCommands<'_>, assets: &AssetServer, is_paused: bool) {
    let mut spawn = |button, action: fn(&mut TickRate)| {
        parent
            .spawn(make_time_button(assets, button))
            .observe(move |_: On<Activate>, mut tick_rate: ResMut<TickRate>| {
                action(&mut tick_rate);
            });
    };

    if is_paused {
        spawn(TimeButton::Play, TickRate::unpause);
        spawn(TimeButton::Step, TickRate::increment_pending);
    } else {
        spawn(TimeButton::Pause, TickRate::pause);
        spawn(TimeButton::Play, |rate| rate.set_tps(4.0));
        spawn(TimeButton::FastForward, |rate| rate.set_tps(16.0));
    }
}

fn make_time_button(assets: &AssetServer, time_button: TimeButton) -> impl Bundle {
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
