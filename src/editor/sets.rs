use bevy::prelude::*;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum EditorUpdateSet {
    Spawn,
    Input,
    Despawn,
}
