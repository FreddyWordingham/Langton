use bevy::prelude::*;

pub const DEFAULT_BUTTON_PADDING: UiRect = UiRect::horizontal(Val::Px(5.0));

pub const BUTTON_IDLE_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BUTTON_HOVERED_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.35, 0.35, 0.35);

pub const DEFAULT_BUTTON_BORDER: UiRect = UiRect::all(Val::Px(2.0));
pub const DEFAULT_BUTTON_BORDER_RADIUS: BorderRadius = BorderRadius {
    top_left: CornerRadius::circular(Val::Px(4.0)),
    top_right: CornerRadius::circular(Val::Px(4.0)),
    bottom_left: CornerRadius::circular(Val::Px(4.0)),
    bottom_right: CornerRadius::circular(Val::Px(4.0)),
};
pub const DEFAULT_BUTTON_BORDER_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
