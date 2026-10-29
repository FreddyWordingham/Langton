use bevy::prelude::*;

use crate::render::Appearance;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Machine {
    Drill,
    Pumpjack,
}

impl Machine {
    pub(crate) const ALL: [Machine; 2] = [Machine::Drill, Machine::Pumpjack];

    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Machine::Drill => "Drill",
            Machine::Pumpjack => "Pumpjack",
        }
    }
}

impl Appearance for Machine {
    fn sprite(&self) -> &'static str {
        match self {
            Machine::Drill => "sprites/machine/drill.png",
            Machine::Pumpjack => "sprites/machine/pumpjack.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Machine::Drill => Color::WHITE,
            Machine::Pumpjack => Color::WHITE,
        }
    }
}
