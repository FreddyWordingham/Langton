use crate::thing::{Item, Machine, Terrain};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EditorTab {
    Terrain,
    Machine,
    Item,
}

impl EditorTab {
    pub(super) const ALL: [Self; 3] = [Self::Terrain, Self::Machine, Self::Item];

    pub(super) fn as_str(&self) -> &'static str {
        match self {
            Self::Terrain => "Terr",
            Self::Machine => "Mach",
            Self::Item => "Item",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EditorSelection {
    Terrain(Terrain),
    Machine(Machine),
    Item(Item),
}
