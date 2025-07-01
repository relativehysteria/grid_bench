pub mod rng;
pub mod grid1;
pub mod grid2;

pub trait GridImpl<T> {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T>;
    fn new(width: usize, height: usize) -> Self;
}
