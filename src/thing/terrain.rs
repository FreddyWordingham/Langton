use bevy::prelude::*;

use crate::{render::Appearance, thing::Item};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Terrain {
    Ore(Item),
    Well(Item),
}

impl Terrain {
    pub(crate) fn is_well(&self) -> bool {
        matches!(self, Terrain::Well(_))
    }

    pub(crate) fn item(&self) -> Item {
        match self {
            Terrain::Ore(item) => *item,
            Terrain::Well(item) => *item,
        }
    }
}

impl Appearance for Terrain {
    fn sprite(&self) -> &'static str {
        match self {
            Terrain::Ore(_) => "sprites/terrain/ore.png",
            Terrain::Well(_) => "sprites/terrain/well.png",
        }
    }

    fn color(&self) -> Color {
        match self {
            Terrain::Ore(item) => item.color(),
            Terrain::Well(item) => item.color(),
        }
    }
}
