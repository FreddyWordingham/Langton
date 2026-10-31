use arc_draw::Draw;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Construct {
    Pumpjack,
    Drill,
}

impl Construct {
    pub const ALL: [Self; 2] = [Self::Pumpjack, Self::Drill];
}

impl Draw for Construct {
    fn name(&self) -> &'static str {
        match self {
            Self::Pumpjack => "Pumpjack",
            Self::Drill => "Drill",
        }
    }

    fn sprite_path(&self) -> &'static str {
        match self {
            Self::Pumpjack => "sprites/tiles/machine/pumpjack.png",
            Self::Drill => "sprites/tiles/machine/drill.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Pumpjack => Color::WHITE,
            Self::Drill => Color::WHITE,
        }
    }
}
