use arc_item::Item;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    Ore(Item),
    Well(Item),
}
