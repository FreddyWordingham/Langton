use std::ops::{Index, IndexMut};

use bevy::prelude::*;

use crate::utils::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    size: UVec2,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(size: UVec2, value: T) -> Self
    where
        T: Clone,
    {
        let len = (size.x as usize)
            .checked_mul(size.y as usize)
            .expect("Grid length cannot fit in usize");
        Self {
            size,
            data: vec![value; len],
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn in_bounds(&self, coord: UVec2) -> bool {
        coord.x < self.size.x && coord.y < self.size.y
    }

    pub fn try_in_bounds(&self, point: IVec2) -> Option<UVec2> {
        UVec2::try_from(point).ok().filter(|&coord| self.in_bounds(coord))
    }

    pub fn index_to_coord(&self, index: usize) -> UVec2 {
        index_to_coord(index, self.size)
    }

    pub fn coord_to_index(&self, coord: UVec2) -> usize {
        coord_to_index(coord, self.size)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn coords(&self) -> impl Iterator<Item = UVec2> {
        iter_coords(self.size)
    }

    pub fn iter_with_coords(&self) -> impl Iterator<Item = (UVec2, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(move |(index, value)| (self.index_to_coord(index), value))
    }
}

impl<T> Index<UVec2> for Grid<T> {
    type Output = T;

    fn index(&self, coord: UVec2) -> &Self::Output {
        let index = self.coord_to_index(coord);
        &self.data[index]
    }
}

impl<T> IndexMut<UVec2> for Grid<T> {
    fn index_mut(&mut self, coord: UVec2) -> &mut Self::Output {
        let index = self.coord_to_index(coord);
        &mut self.data[index]
    }
}
