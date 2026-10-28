use bevy::prelude::*;

use crate::thing::{Item, Machine, Terrain};

#[derive(Message)]
pub struct SpawnTerrain {
    pub terrain: Terrain,
    pub location: IVec2,
}

#[derive(Message)]
pub struct SpawnMachine {
    pub machine: Machine,
    pub location: IVec2,
}

#[derive(Message)]
pub struct SpawnItem {
    pub item: Item,
    pub location: IVec2,
}
