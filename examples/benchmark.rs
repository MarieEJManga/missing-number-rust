use missing_numbers::math::find_missing_math;
use missing_numbers::sorting::find_missing_sorting;
use missing_numbers::xor::find_missing_xor;
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let n: u64 = 1_000_000;
    let missing: u64 = 500_000;
    let iterations = 20;

    let numbers: Vec<u64> = (1..=n)
        .filter(|&x| x != missing)
        .collect();

    let xor_numbers: Vec<u32> = numbers.iter().map(|&x| x as u32).collect();

    let mut math_total = 0.0;
    let mut sorting_total = 0.0;
    let mut xor_total = 0.0;

    for _ in 0..iterations {
        // Mathematical Sum
        let start = Instant::now();
        let math_result = find_missing_math(n, &numbers);
        math_total += start.elapsed().as_secs_f64();
        black_box(math_result);

        // Sorting
        let sorting_numbers = numbers.clone();

        let start = Instant::now();
        let sorting_result =
            find_missing_sorting(n, sorting_numbers);
        sorting_total += start.elapsed().as_secs_f64();
        black_box(sorting_result);

        // XOR
        let start = Instant::now();
        let xor_result = find_missing_xor(n as u32, &xor_numbers);
        xor_total += start.elapsed().as_secs_f64();
        black_box(xor_result);
    }

    assert_eq!(missing, find_missing_math(n, &numbers));
    assert_eq!(
        missing as u32,
        find_missing_sorting(n, numbers.clone()) as u32
    );
    assert_eq!(missing as u32, find_missing_xor(n as u32, &xor_numbers));

    println!("All three algorithms passed correctness checks.");
    println!("Input size: {}", n);
    println!("Iterations: {}", iterations);

    println!(
        "Mathematical Sum average: {:.3} ms",
        math_total * 1000.0 / iterations as f64
    );

    println!(
        "Sorting average: {:.3} ms",
        sorting_total * 1000.0 / iterations as f64
    );

    println!(
        "XOR average: {:.3} ms",
        xor_total * 1000.0 / iterations as f64
    );
}