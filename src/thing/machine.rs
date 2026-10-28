use bevy::prelude::*;

use crate::render::Appearance;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Machine {
    Pumpjack,
}

impl Appearance for Machine {
    fn sprite(&self) -> &'static str {
        match self {
            Machine::Pumpjack => "sprites/machine/pumpjack.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Machine::Pumpjack => Color::WHITE,
        }
    }
}
