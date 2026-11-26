
# Missing Number – Rust Project

## Overview

This project solves the CSES problem Missing Number (1083).

The task is to find the one missing number from a sequence containing numbers from 1 to n.

Three algorithms are implemented:
1. Mathematical Sum
2. Sorting
3. XOR (Bitwise Operation)

## Project Structure

- src/main.rs – Main program.
- src/math.rs – Mathematical Sum algorithm.
- src/sorting.rs – Sorting algorithm.
- src/xor.rs – XOR algorithm.
- src/lib.rs – Exports the algorithms.
- tests/algorithm_tests.rs – Algorithm tests.
- tests/cses_tests.rs – CSES test integration.
- benches/benchmark.rs – Criterion benchmark.
- BENCHMARK.md – Benchmark results.
- missing_number_cses.rs – Standalone CSES submission.

## How to Run

Run the program:

```bash
cargo run
```

Run all tests:

```bash
cargo test
```

Run the Criterion benchmark:

```bash
cargo bench
```

## Benchmark Results

- Input size: 1,000,000 numbers.
- Framework: Criterion 0.5.
- Samples: 100 measurements per algorithm.
- Build mode: Release.

| Algorithm | Estimated time |
|---|---:|
| Mathematical Sum | 107.62 µs |
| Sorting | 598.58 µs |
| XOR | 119.21 µs |

Mathematical Sum was the fastest in this benchmark. XOR was close, while Sorting was slower.

## CSES Submission

Problem: https://cses.fi/problemset/task/1083/

Submission status: Accepted.

Accepted submission: https://cses.fi/problemset/result/19403038/
