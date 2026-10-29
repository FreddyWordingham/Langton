use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Tick(u64);

impl Tick {
    pub(crate) fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn tick(&self) -> u64 {
        self.0
    }
}
