use arc_theme::{BUTTON_HOVERED_COLOR, BUTTON_IDLE_COLOR, BUTTON_PRESSED_COLOR};
use bevy::{picking::hover::Hovered, prelude::*, ui::Pressed, ui_widgets::Button};

pub(crate) fn update_button_colors(mut buttons: Query<(&Hovered, Has<Pressed>, &mut BackgroundColor), With<Button>>) {
    for (hovered, pressed, mut background) in &mut buttons {
        let color = match (pressed, hovered.get()) {
            (true, _) => BUTTON_PRESSED_COLOR,
            (false, true) => BUTTON_HOVERED_COLOR,
            (false, false) => BUTTON_IDLE_COLOR,
        };

        if background.0 != color {
            background.0 = color;
        }
    }
}
