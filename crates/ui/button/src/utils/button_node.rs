use arc_theme::{DEFAULT_BUTTON_BORDER, DEFAULT_BUTTON_BORDER_RADIUS, DEFAULT_BUTTON_PADDING};
use bevy::prelude::*;

pub fn button_node() -> Node {
    Node {
        border: DEFAULT_BUTTON_BORDER,
        border_radius: DEFAULT_BUTTON_BORDER_RADIUS,
        padding: DEFAULT_BUTTON_PADDING,
        ..default()
    }
}
