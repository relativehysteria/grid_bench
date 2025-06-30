use std::sync::Arc;
use std::thread;
use std::time::Instant;
use std::arch::x86_64::_rdtsc;

use asdf::grid1::Grid as Grid1;
use asdf::grid2::Grid as Grid2;
use asdf::rng::Rng;

const X: usize = 4096;
const Y: usize = 4096;

fn main() {
    // Initialize RNG with timestamp counter as seed
    let seed = unsafe { _rdtsc() as usize };
    let mut rng = Rng::new(seed);

    // Create the grids
    let mut grid1: Grid1<usize> = Grid1::new(X, Y);
    let mut grid2: Grid2<usize> = Grid2::new(X, Y);

    // Determine number of coordinates to sample
    let n_coords = (X * Y) / (((X + Y) / 2) / 4);

    // Generate coordinate vectors
    let mut generate_coords = || {
        (0..n_coords)
            .map(|_| (rng.range(0, X - 1), rng.range(0, Y - 1)))
            .collect::<Vec<_>>()
    };

    let pre_check = Arc::new(generate_coords());
    let check = Arc::new(generate_coords());

    // Spawn benchmark thread for grid2
    let pre_check2 = Arc::clone(&pre_check);
    let check2 = Arc::clone(&check);
    let g2 = thread::spawn(move || {
        for &(x, y) in &*pre_check2 {
            *grid2.get_mut(x, y).unwrap() = x + y;
        }

        let start = Instant::now();
        for &(x, y) in &*check2 {
            *grid2.get_mut(x, y).unwrap() = x + y;
        }
        start.elapsed()
    });

    // Spawn benchmark thread for grid1
    let g1 = thread::spawn(move || {
        for &(x, y) in &*pre_check {
            *grid1.get_mut(x, y).unwrap() = x + y;
        }

        let start = Instant::now();
        for &(x, y) in &*check {
            *grid1.get_mut(x, y).unwrap() = x + y;
        }
        start.elapsed()
    });

    // Wait for the threads to complete
    let g1 = g1.join().unwrap();
    let g2 = g2.join().unwrap();

    println!("1D grid: {g1:?}");
    println!("2D grid: {g2:?}");
}
