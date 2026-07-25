use std::{hint::black_box, time::Instant};

use rust_course_tooling::{binary_contains, linear_contains};

fn main() {
    let values: Vec<u64> = (0..100_000).collect();
    let target = black_box(99_999);

    let started = Instant::now();
    let linear_result = linear_contains(black_box(&values), target);
    let linear_elapsed = started.elapsed();

    let started = Instant::now();
    let binary_result = binary_contains(black_box(&values), target);
    let binary_elapsed = started.elapsed();

    assert_eq!(linear_result, binary_result);
    println!("한 번 선형 검색: {linear_elapsed:?}");
    println!("한 번 이진 검색: {binary_elapsed:?}");
}
