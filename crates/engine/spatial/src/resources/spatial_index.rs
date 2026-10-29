use std::{collections::HashMap, marker::PhantomData};

use bevy::prelude::*;

#[derive(Resource)]
pub struct SpatialIndex<T> {
    _marker: PhantomData<T>,
    particles: HashMap<IVec2, Entity>,
}

impl<T> Default for SpatialIndex<T> {
    fn default() -> Self {
        SpatialIndex {
            _marker: PhantomData,
            particles: HashMap::new(),
        }
    }
}

impl<T> SpatialIndex<T> {
    pub fn insert(&mut self, entity: Entity, position: IVec2) {
        if self.particles.contains_key(&position) {
            panic!("Position {:?} is already occupied", position);
        }
        self.particles.insert(position, entity);
    }

    pub fn remove(&mut self, position: IVec2) {
        self.particles.remove(&position);
    }

    pub fn transfer(&mut self, from: IVec2, to: IVec2) {
        if !self.particles.contains_key(&from) {
            panic!("No entity at position {:?}", from);
        }
        if self.particles.contains_key(&to) {
            panic!("Position {:?} is already occupied", to);
        }
        let entity = self.particles.remove(&from).unwrap();
        self.particles.insert(to, entity);
    }

    pub fn get(&self, position: &IVec2) -> Option<Entity> {
        self.particles.get(position).copied()
    }
}
