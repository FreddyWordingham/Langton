use bevy::prelude::*;

use crate::lifecycle::ApplyTo;

#[derive(Message)]
pub struct SpawnThing<T: ApplyTo> {
    pub thing: T,
    pub location: IVec2,
}
