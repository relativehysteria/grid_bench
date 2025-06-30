#![allow(dead_code)]

use std::slice::{Iter, IterMut};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Grid<T: Debug> {
    pub data: Vec<Vec<T>>,
}

impl<T: Debug> Grid<T> {
    /// Creates a new grid with default values (requires `T: Default`).
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        let data = (0..height)
            .map(|_| vec![T::default(); width])
            .collect::<Vec<Vec<T>>>();
        Grid { data }
    }

    /// Creates a grid from raw data (panics if data length mismatches
    /// dimensions).
    pub fn from_data(data: Vec<Vec<T>>) -> Self {
        Grid { data }
    }

    /// Returns the grid width.
    #[inline(always)]
    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    /// Returns the grid height.
    #[inline(always)]
    pub fn height(&self) -> usize {
        self.data.len()
    }

    /// Access element safely with bounds checks.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).map(|row| row.get(x)).flatten()
    }

    /// Mutably access element safely with bounds checks.
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y).map(|row| row.get_mut(x)).flatten()
    }

    /// Unchecked access (unsafe: caller must ensure `x < width` and `y <
    /// height`).
    ///
    /// # Safety
    /// `x` must be in [0, width-1] and `y` in [0, height-1].
    #[inline(always)]
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        unsafe { self.data.get_unchecked(y).get_unchecked(x) }
    }

    /// Unchecked mutable access (same safety conditions as `get_unchecked`).
    #[inline(always)]
    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        unsafe { self.data.get_unchecked_mut(y).get_unchecked_mut(x) }
    }

    /// Returns a reference to a row slice (panics if `y` is out-of-bounds).
    #[inline]
    pub fn row(&self, y: usize) -> &[T] {
        assert!(y < self.height(), "Row index out of bounds");
        unsafe { self.data.get_unchecked(y) }
    }

    /// Returns a mutable reference to a row slice (panics if `y` is
    /// out-of-bounds).
    #[inline]
    pub fn row_mut(&mut self, y: usize) -> &mut [T] {
        assert!(y < self.height(), "Row index out of bounds");
        unsafe { self.data.get_unchecked_mut(y) }
    }

    /// Returns an iterator over rows as slices.
    #[inline]
    pub fn rows(&self) -> Iter<'_, Vec<T>> {
        self.data.iter()
    }

    /// Returns a mutable iterator over rows as slices.
    #[inline]
    pub fn rows_mut(&mut self) -> IterMut<'_, Vec<T>> {
        self.data.iter_mut()
    }
}
