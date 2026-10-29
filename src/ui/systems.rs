use bevy::{picking::hover::Hovered, prelude::*, ui::Pressed, ui_widgets::Button};

use crate::ui::{HOVERED_COLOR, IDLE_COLOR, PRESSED_COLOR};

pub(super) fn update_button_colors(mut button_q: Query<(&Hovered, Has<Pressed>, &mut BackgroundColor), With<Button>>) {
    for (hovered, pressed, mut background) in &mut button_q {
        match (hovered.get(), pressed) {
            (_, true) => {
                background.0 = PRESSED_COLOR;
            }
            (true, false) => {
                background.0 = HOVERED_COLOR;
            }
            (false, false) => {
                background.0 = IDLE_COLOR;
            }
        }
    }
}
