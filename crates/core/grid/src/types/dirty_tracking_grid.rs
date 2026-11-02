use std::ops::{Index, IndexMut};

use bevy::{platform::collections::HashSet, prelude::*};

use crate::types::Grid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirtyTrackingGrid<T> {
    grid: Grid<T>,
    dirty: HashSet<UVec2>,
}

impl<T> DirtyTrackingGrid<T> {
    pub fn new(size: UVec2, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            grid: Grid::new(size, value),
            dirty: HashSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    pub fn size(&self) -> UVec2 {
        self.grid.size()
    }

    pub fn in_bounds(&self, coord: UVec2) -> bool {
        self.grid.in_bounds(coord)
    }

    pub fn try_in_bounds(&self, point: IVec2) -> Option<UVec2> {
        self.grid.try_in_bounds(point)
    }

    pub fn index_to_coord(&self, index: usize) -> UVec2 {
        self.grid.index_to_coord(index)
    }

    pub fn coord_to_index(&self, coord: UVec2) -> usize {
        self.grid.coord_to_index(coord)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter()
    }

    pub fn coords(&self) -> impl Iterator<Item = UVec2> {
        self.grid.coords()
    }

    pub fn iter_with_coords(&self) -> impl Iterator<Item = (UVec2, &T)> {
        self.grid.iter_with_coords()
    }

    pub fn mark_dirty(&mut self, coord: UVec2) {
        if self.in_bounds(coord) {
            self.dirty.insert(coord);
        }
    }

    pub fn is_dirty(&self) -> bool {
        !self.dirty.is_empty()
    }

    pub fn drain_dirty(&mut self) -> impl Iterator<Item = UVec2> + '_ {
        self.dirty.drain()
    }
}

impl<T> Index<UVec2> for DirtyTrackingGrid<T> {
    type Output = T;

    fn index(&self, coord: UVec2) -> &Self::Output {
        &self.grid[coord]
    }
}

impl<T> IndexMut<UVec2> for DirtyTrackingGrid<T> {
    fn index_mut(&mut self, coord: UVec2) -> &mut Self::Output {
        self.mark_dirty(coord);
        &mut self.grid[coord]
    }
}
