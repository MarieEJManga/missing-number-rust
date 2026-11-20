# Missing Number – Rust Project

## Overview

This project solves the CSES problem **Missing Number (1083)**.

The task is to find the one missing number from a sequence containing numbers from 1 to n.

Three algorithms are implemented and compared:

1. **Mathematical Sum Algorithm**
   - Calculates the expected sum from 1 to n.
   - Subtracts the sum of the given numbers.
   - Time complexity: O(n).
   - Uses u64 to safely calculate the sum.

2. **Sorting Algorithm**
   - Sorts the given numbers.
   - Checks which number is missing.
   - Time complexity: O(n log n).

3. **XOR Algorithm**
   - Uses bitwise XOR operations to cancel matching numbers.
   - The remaining value is the missing number.
   - Time complexity: O(n).
   - Uses u32 because the values fit within the CSES problem constraints.

## Project Structure

- `src/main.rs` – Main program.
- `src/math.rs` – Mathematical sum algorithm.
- `src/sorting.rs` – Sorting algorithm.
- `src/xor.rs` – XOR algorithm.
- `src/lib.rs` – Exports the algorithms.
- `tests/algorithm_tests.rs` – Algorithm tests.
- `tests/cses_tests.rs` – CSES test integration.
- `examples/benchmark.rs` – Benchmark for all three algorithms.
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

Benchmark setup:
- Input size: 1,000,000 numbers.
- Iterations: 20.
- Build mode: Release.

| Algorithm | Average execution time |
|---|---:|
| Mathematical Sum | 0.998 ms |
| Sorting | 1.870 ms |
| XOR | 0.632 ms |

The XOR algorithm was the fastest in this benchmark.

It was approximately:
- 1.58 times faster than Mathematical Sum.
- 2.96 times faster than Sorting.

The benchmark measures the algorithm execution time. Preparation such as cloning and converting the input is outside the measured time.

Benchmark timings may vary depending on the computer and system conditions.

## CSES Submission

Problem: https://cses.fi/problemset/task/1083/

Submission status: Accepted.

Accepted submission: https://cses.fi/problemset/result/19403038/