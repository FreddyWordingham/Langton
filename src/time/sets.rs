use bevy::prelude::*;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum TimeStartupSet {
    Spawn,
}

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum TimeUpdateSet {
    Input,
    Sync,
}
