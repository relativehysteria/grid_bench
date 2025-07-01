use std::ops::Range;
use std::time::{Instant, Duration};
use std::arch::x86_64::_rdtsc;
use asdf::grid1::Grid as Grid1;
use asdf::grid2::Grid as Grid2;
use asdf::rng::Rng;
use asdf::GridImpl;

/// The range of exponents used to generate test grid sizes.
/// Each value `e` in this range results in a base size of `2^e`,
/// which is then used to create various (width, height) combinations
/// for benchmarking grid implementations.
const EXP_RANGE: std::ops::Range<usize> = 8..15;

/// Run each benchmark this many times and average out the results
const N_RUNS: usize = 20;

type Coord = (usize, usize);

fn bench<T: GridImpl<usize>>(grid: &mut T, pre: &[Coord], bench: &[Coord])
    -> Duration
{
    // Pre-heat the CPU
    for &(x, y) in pre.iter() {
        *grid.get_mut(x, y).unwrap() = x + y;
    }

    // Start the benchmark
    let mut total = Duration::ZERO;

    for _ in 0..N_RUNS {
        let start = Instant::now();
        std::hint::black_box({
            for &(x, y) in bench.iter() {
                *grid.get_mut(x, y).unwrap() = x + y;
            }
        });
        total += start.elapsed()
    }
    total / N_RUNS as u32
}

// Generate the dimension dataset that will be used to test the grids
fn generate_dimensions(exponent_range: Range<usize>) -> Vec<Coord> {
    let mut test_dims: Vec<Coord> = Vec::new();

    for exponent in exponent_range {
        let base = 1 << exponent;

        // Vary X and Y dimensions independently
        let dimensions = [
            // Standard power-of-two based dimensions
            (base, base),         // 1:1 (square)
            (base, base / 2),     // 2:1 (wide rectangle)
            (base / 2, base),     // 1:2 (tall rectangle)
            (base, base * 2),     // 1:2 (larger tall)
            (base * 2, base),     // 2:1 (larger wide)

            // Additional ratios for more variety
            (base * 3, base * 2), // 3:2
            (base * 2, base * 3), // 2:3
            (base * 4, base / 2), // 8:1 (extreme wide)
            (base / 2, base * 4), // 1:8 (extreme tall)

            // Non-power-of-two dimensions
            (base * 3 / 2, base), // 3:2 (approximately)
            (base, base * 3 / 2), // 2:3 (approximately)
            (base * 5 / 4, base * 3 / 4), // 5:3
        ];
        test_dims.extend(dimensions.iter());
    }

    test_dims
}

fn main() {
    println!("Width   Height              1D  2D");

    // Start up the RNG
    let seed = unsafe { _rdtsc() as usize };
    let mut rng = Rng::new(seed);

    // Perform the benchmark
    for (x, y) in generate_dimensions(EXP_RANGE) {
        // Calculate number of coordinates based on grid dimensions
        let n_coords = (16 * x * y) / (x + y);

        // Helper to generate coordinate vectors
        let mut generate_coords = || {
            (0..n_coords)
                .map(|_| (rng.range(0, x - 1), rng.range(0, y - 1)))
                .collect::<Vec<_>>()
        };

        // Coordinates used for the benchmark and for the warmup round
        let bench_coords = generate_coords();
        let mut warmup_coords = generate_coords();
        warmup_coords.extend(generate_coords());

        // Run the benchmarks
        let g1 = bench(&mut Grid1::new(x, y), &warmup_coords, &bench_coords);
        let g2 = bench(&mut Grid2::new(x, y), &warmup_coords, &bench_coords);

        println!("{:>6}  {:>6}  {:>14?}  {:<14?}", x, y, g1, g2);
    }
}
