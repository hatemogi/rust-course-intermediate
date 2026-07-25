use std::{hint::black_box, time::Instant};

use rust_course_tooling::{binary_contains, linear_contains};

// #region measure
const ITERATIONS: u32 = 10_000;
const WARMUP_ITERATIONS: u32 = 1_000;

fn measure(label: &str, mut operation: impl FnMut() -> bool) {
    for _ in 0..WARMUP_ITERATIONS {
        black_box(operation());
    }

    let started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(operation());
    }
    let elapsed = started.elapsed();
    let nanoseconds_per_iteration = elapsed.as_secs_f64() * 1_000_000_000.0 / f64::from(ITERATIONS);

    println!("{label}: {nanoseconds_per_iteration:.2} ns/회 ({ITERATIONS}회 합계 {elapsed:?})");
}
// #endregion measure

fn main() {
    let values: Vec<u64> = (0..10_000).collect();
    let target = black_box(9_999);

    measure("선형 검색", || {
        linear_contains(black_box(&values), target)
    });
    measure("이진 검색", || {
        binary_contains(black_box(&values), target)
    });
}
