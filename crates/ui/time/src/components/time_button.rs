use bevy::prelude::*;

#[derive(Component)]
pub(crate) enum TimeButton {
    Pause,
    Play,
    FastForward,
    Step,
}

impl TimeButton {
    pub(crate) fn icon_path(&self) -> &'static str {
        match self {
            TimeButton::Pause => "sprites/icons/time/pause.png",
            TimeButton::Play => "sprites/icons/time/play.png",
            TimeButton::FastForward => "sprites/icons/time/fast_forward.png",
            TimeButton::Step => "sprites/icons/time/step.png",
        }
    }
}
