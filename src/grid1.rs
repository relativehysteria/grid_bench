#![allow(dead_code)]

use std::ops::{Index, IndexMut};
use std::slice::{ChunksExact, ChunksExactMut};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Grid<T: Debug> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Debug> Grid<T> {
    /// Creates a new grid with default values (requires `T: Default`).
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        let data = vec![T::default(); width * height];
        Grid { width, height, data }
    }

    /// Creates a grid from raw data (panics if data length mismatches dimensions).
    pub fn from_data(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(width * height, data.len(),
            "Data length must match grid dimensions");
        Grid { width, height, data }
    }

    /// Returns the grid width.
    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the grid height.
    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Access element safely with bounds checks.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        (x < self.width && y < self.height)
            .then(|| unsafe { self.get_unchecked(x, y) })
    }

    /// Mutably access element safely with bounds checks.
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        (x < self.width && y < self.height)
            .then(|| unsafe { self.get_unchecked_mut(x, y) })
    }

    /// Unchecked access (unsafe: caller must ensure `x < width` and `y <
    /// height`).
    ///
    /// # Safety
    /// `x` must be in [0, width-1] and `y` in [0, height-1].
    #[inline(always)]
    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        debug_assert!(x < self.width && y < self.height);
        unsafe { self.data.get_unchecked(y * self.width + x) }
    }

    /// Unchecked mutable access (same safety conditions as `get_unchecked`).
    #[inline(always)]
    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        debug_assert!(x < self.width && y < self.height);
        unsafe { self.data.get_unchecked_mut(y * self.width + x) }
    }

    /// Returns a reference to a row slice (panics if `y` is out-of-bounds).
    #[inline]
    pub fn row(&self, y: usize) -> &[T] {
        assert!(y < self.height, "Row index out of bounds");
        let start = y * self.width;
        &self.data[start..start + self.width]
    }

    /// Returns a mutable reference to a row slice (panics if `y` is
    /// out-of-bounds).
    #[inline]
    pub fn row_mut(&mut self, y: usize) -> &mut [T] {
        assert!(y < self.height, "Row index out of bounds");
        let start = y * self.width;
        &mut self.data[start..start + self.width]
    }

    /// Returns an iterator over rows as slices.
    #[inline]
    pub fn rows(&self) -> ChunksExact<'_, T> {
        self.data.chunks_exact(self.width)
    }

    /// Returns a mutable iterator over rows as slices.
    #[inline]
    pub fn rows_mut(&mut self) -> ChunksExactMut<'_, T> {
        self.data.chunks_exact_mut(self.width)
    }

    /// Converts grid to a flat slice of the underlying data.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Converts grid to a mutable flat slice of the underlying data.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}

// Indexing operators for convenient syntax: grid[(x, y)]
impl<T: Debug> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    #[inline]
    fn index(&self, (x, y): (usize, usize)) -> &T {
        self.get(x, y).expect("Index out of bounds")
    }
}

impl<T: Debug> IndexMut<(usize, usize)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        self.get_mut(x, y).expect("Index out of bounds")
    }
}
