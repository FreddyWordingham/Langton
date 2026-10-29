use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};

use crate::time::{PendingSteps, TickRate, TimeMenu, TimeSpeed, TimeSpeedButton};

pub(super) fn update_tick_rate(
    tick_rate: Res<TickRate>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut fixed_time: ResMut<Time<Fixed>>,
) {
    if let Some(tps) = tick_rate.0 {
        fixed_time.set_timestep_hz(tps);
        virtual_time.unpause();
    } else {
        virtual_time.pause();
    }
}

pub(super) fn step_time(
    mut pending: ResMut<PendingSteps>,
    mut virtual_time: ResMut<Time<Virtual>>,
    fixed_time: Res<Time<Fixed>>,
    mut time: ResMut<Time>,
) {
    if !virtual_time.is_paused() {
        pending.0 = 0;
        return;
    }

    if pending.0 > 0 {
        pending.0 -= 1;
        // Inject one timestep after Bevy updates the clocks, before its fixed loop.
        virtual_time.advance_by(fixed_time.timestep());
        *time = virtual_time.as_generic();
    }
}

pub(super) fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            TimeMenu,
            Node {
                position_type: PositionType::Absolute,
                top: px(0.0),
                width: percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(5.0),
                    ..default()
                })
                .with_children(|parent| {
                    for speed in TimeSpeed::ALL {
                        parent
                            .spawn((
                                TimeSpeedButton(speed),
                                Button,
                                Hovered::default(),
                                Node {
                                    flex_grow: 1.0,
                                    flex_basis: px(0.0),
                                    min_width: px(0.0),
                                    height: px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                children![Text::new(speed.as_str())],
                            ))
                            .observe(
                                |event: On<Activate>,
                                 mut tick_rate: ResMut<TickRate>,
                                 mut pending: ResMut<PendingSteps>,
                                 button_q: Query<&TimeSpeedButton>| {
                                    let Ok(button) = button_q.get(event.entity) else {
                                        return;
                                    };
                                    tick_rate.0 = button.0.tps();
                                    pending.0 = 0;
                                },
                            );
                    }
                    parent
                        .spawn((
                            Button,
                            Hovered::default(),
                            Node {
                                height: px(30.0),
                                padding: UiRect::horizontal(px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            children![Text::new("Step")],
                        ))
                        .observe(
                            |_: On<Activate>, mut tick_rate: ResMut<TickRate>, mut pending: ResMut<PendingSteps>| {
                                tick_rate.0 = None;
                                pending.0 = pending.0.saturating_add(1);
                            },
                        );
                });
        });
}
