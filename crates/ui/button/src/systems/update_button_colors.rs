use bevy::{picking::hover::Hovered, prelude::*, ui::Pressed, ui_widgets::Button};

use crate::constants::{HOVERED_COLOR, IDLE_COLOR, PRESSED_COLOR};

pub(crate) fn update_button_colors(mut buttons: Query<(&Hovered, Has<Pressed>, &mut BackgroundColor), With<Button>>) {
    for (hovered, pressed, mut background) in &mut buttons {
        let color = match (pressed, hovered.get()) {
            (true, _) => PRESSED_COLOR,
            (false, true) => HOVERED_COLOR,
            (false, false) => IDLE_COLOR,
        };

        if background.0 != color {
            background.0 = color;
        }
    }
}
