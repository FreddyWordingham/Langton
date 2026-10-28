use std::{collections::HashMap, marker::PhantomData};

use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct SpatialIndex<T> {
    particles: HashMap<IVec2, Entity>,
    _marker: PhantomData<T>,
}

impl<T> Default for SpatialIndex<T> {
    fn default() -> Self {
        SpatialIndex {
            particles: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl<T> SpatialIndex<T> {
    pub(crate) fn insert(&mut self, entity: Entity, position: IVec2) {
        if self.particles.contains_key(&position) {
            panic!("Position {:?} is already occupied", position);
        }
        self.particles.insert(position, entity);
    }

    pub(crate) fn remove(&mut self, position: IVec2) {
        self.particles.remove(&position);
    }

    pub(crate) fn transfer(&mut self, from: IVec2, to: IVec2) {
        if !self.particles.contains_key(&from) {
            panic!("No entity at position {:?}", from);
        }
        if self.particles.contains_key(&to) {
            panic!("Position {:?} is already occupied", to);
        }
        let entity = self.particles.remove(&from).unwrap();
        self.particles.insert(to, entity);
    }

    pub(crate) fn get(&self, position: &IVec2) -> Option<Entity> {
        self.particles.get(position).copied()
    }
}
