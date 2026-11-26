use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use missing_numbers::math::find_missing_math;
use missing_numbers::sorting::find_missing_sorting;
use missing_numbers::xor::find_missing_xor;

fn benchmark_algorithms(c: &mut Criterion) {
    let n: u64 = 1_000_000;
    let missing: u64 = 500_000;

    let numbers: Vec<u64> = (1..=n)
        .filter(|&x| x != missing)
        .collect();

    let xor_numbers: Vec<u32> = numbers.iter().map(|&x| x as u32).collect();

    // Check that all three algorithms return the correct answer.
    assert_eq!(missing, find_missing_math(n, &numbers));
    assert_eq!(missing, find_missing_sorting(n, numbers.clone()));
    assert_eq!(
        missing as u32,
        find_missing_xor(n as u32, &xor_numbers)
    );

    // Benchmark the mathematical sum algorithm.
    c.bench_function("Mathematical Sum", |b| {
        b.iter(|| find_missing_math(black_box(n), black_box(&numbers)))
    });

    // Benchmark sorting. The input copy is prepared outside the measured time.
    c.bench_function("Sorting", |b| {
        b.iter_batched(
            || numbers.clone(),
            |input| find_missing_sorting(black_box(n), black_box(input)),
            BatchSize::SmallInput,
        )
    });

    // Benchmark the XOR algorithm.
    c.bench_function("XOR", |b| {
        b.iter(|| find_missing_xor(black_box(n as u32), black_box(&xor_numbers)))
    });
}

criterion_group!(benches, benchmark_algorithms);
criterion_main!(benches);