use bevy::{picking::hover::Hovered, prelude::*, ui::Pressed, ui_widgets::Button};

use crate::ui::{ButtonSelected, HOVERED_COLOR, IDLE_COLOR, PRESSED_COLOR, SELECTED_COLOR, SELECTED_HOVERED_COLOR};

pub(super) fn update_button_colors(
    mut buttons: Query<(&Hovered, Has<Pressed>, Option<&ButtonSelected>, &mut BackgroundColor), With<Button>>,
) {
    for (hovered, pressed, selected, mut background) in &mut buttons {
        let selected = selected.is_some_and(|state| state.0);

        let color = match (pressed, selected, hovered.get()) {
            (true, _, _) => PRESSED_COLOR,
            (false, true, true) => SELECTED_HOVERED_COLOR,
            (false, true, false) => SELECTED_COLOR,
            (false, false, true) => HOVERED_COLOR,
            (false, false, false) => IDLE_COLOR,
        };

        if background.0 != color {
            background.0 = color;
        }
    }
}
