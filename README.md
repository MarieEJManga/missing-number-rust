# Missing Number – Rust Project

## Overview

This project solves the CSES problem **Missing Number (1083)**.

The task is to find the one missing number from a sequence containing numbers from 1 to n.

Two algorithms are implemented and compared:

1. **Mathematical Sum Algorithm**
   - Calculates the expected sum from 1 to n.
   - Subtracts the sum of the given numbers.
   - Time complexity: O(n).

2. **Sorting Algorithm**
   - Sorts the given numbers.
   - Checks which number is missing.
   - Time complexity: O(n log n).

## Project Structure

- `src/main.rs` – Main program.
- `src/math.rs` – Mathematical sum algorithm.
- `src/sorting.rs` – Sorting algorithm.
- `tests/algorithm_tests.rs` – Algorithm tests.
- `tests/cses_tests.rs` – CSES test integration.
- `examples/benchmark.rs` – Benchmark for both algorithms.
- `BENCHMARK.md` – Benchmark results and interpretation.
- `missing_number_cses.rs` – Standalone source submitted to CSES.

## How to Run

Run the program:

```bash
cargo run
```

Run all tests:

```bash
cargo test
```

Run the benchmark:

```bash
cargo run --release --example benchmark
```

## Benchmark Results

Input size: 1,000,000 numbers  
Iterations: 20

- Mathematical Sum: 0.972 ms average
- Sorting: 2.596 ms average

The mathematical sum algorithm was approximately 2.7 times faster than the sorting algorithm in this benchmark.

Benchmark timings may vary depending on the computer and system conditions.

## CSES Submission

Problem: https://cses.fi/problemset/task/1083/

Submission status: Accepted.

Accepted submission: https://cses.fi/problemset/result/19403038/