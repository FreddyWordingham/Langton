use arc_theme::{BUTTON_IDLE_COLOR, DEFAULT_BUTTON_BORDER_COLOR};
use bevy::{picking::hover::Hovered, prelude::*, ui_widgets::Button};

pub fn button(node: Node) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        node,
        BorderColor::from(DEFAULT_BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_IDLE_COLOR),
    )
}
