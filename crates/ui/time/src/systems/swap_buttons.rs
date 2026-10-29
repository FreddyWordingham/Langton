use arc_tick::TickRate;
use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::components::{TimeButton, TimeMenu};

pub(crate) fn swap_buttons(
    mut commands: Commands,
    assets: Res<AssetServer>,
    tick_rate: Res<TickRate>,
    mut time_menu_q: Query<(Entity, &mut TimeMenu)>,
) {
    for (ent, mut time_menu) in &mut time_menu_q {
        if time_menu.0 == tick_rate.is_paused() {
            continue;
        }

        time_menu.0 = tick_rate.is_paused();
        commands.entity(ent).despawn_children().with_children(|parent| {
            if tick_rate.is_paused() {
                parent.spawn(make_time_button(&assets, TimeButton::Play)).observe(
                    |_: On<Activate>, mut tick_rate: ResMut<TickRate>| {
                        tick_rate.unpause();
                    },
                );
                parent.spawn(make_time_button(&assets, TimeButton::Step)).observe(
                    |_: On<Activate>, mut tick_rate: ResMut<TickRate>| {
                        tick_rate.increment_pending();
                    },
                );
            } else {
                parent.spawn(make_time_button(&assets, TimeButton::Pause)).observe(
                    |_: On<Activate>, mut tick_rate: ResMut<TickRate>| {
                        tick_rate.pause();
                    },
                );
                parent.spawn(make_time_button(&assets, TimeButton::Play)).observe(
                    |_: On<Activate>, mut tick_rate: ResMut<TickRate>| {
                        tick_rate.set_tps(4.0);
                    },
                );
                parent.spawn(make_time_button(&assets, TimeButton::FastForward)).observe(
                    |_: On<Activate>, mut tick_rate: ResMut<TickRate>| {
                        tick_rate.set_tps(16.0);
                    },
                );
            }
        });
    }
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
