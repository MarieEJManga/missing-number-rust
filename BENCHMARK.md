# Missing Number – Benchmark Report

## 1. Benchmark Setup

Three algorithms were implemented to solve the Missing Number problem:

- Mathematical Sum
- Sorting
- XOR (Bitwise Operation)

The benchmark was performed using:

- Input size: 1,000,000 numbers
- Benchmark framework: Criterion 0.5
- Samples: 100 measurements per algorithm
- Build mode: Release
- Programming language: Rust

All three algorithms passed correctness checks before benchmarking.

## 2. Benchmark Results

The following results were produced by Criterion:

| Algorithm | Estimated time |
|---|---:|
| Mathematical Sum | 107.62 µs |
| Sorting | 598.58 µs |
| XOR | 119.21 µs |

Criterion also reported some measurement outliers. These are normal variations that can occur during benchmarking.

## 3. Time and Space Complexity

**Mathematical Sum**
- Time complexity: O(n)
- Auxiliary space complexity: O(1)

**Sorting**
- Time complexity: O(n log n)
- Additional space complexity: Depends on the sorting implementation.

**XOR**
- Time complexity: O(n)
- Auxiliary space complexity: O(1)

## 4. Interpretation

The Mathematical Sum algorithm was the fastest in this benchmark, with an estimated execution time of 107.62 µs.

The XOR algorithm was very close, taking 119.21 µs. The Sorting algorithm was the slowest, taking 598.58 µs.

The Mathematical Sum algorithm uses a formula to calculate the expected sum and subtracts the actual sum to find the missing number.

The XOR algorithm uses bitwise operations to cancel out matching numbers and identify the missing number.

The Sorting algorithm sorts the numbers before searching for the missing value, which requires more processing time.

In this benchmark, Mathematical Sum was approximately 1.11 times faster than XOR and 5.56 times faster than Sorting.

Benchmark results can vary depending on the computer and system conditions.

## 5. Conclusion

All three algorithms correctly solve the Missing Number problem.

Mathematical Sum performed best in this Criterion benchmark, while XOR had a very similar execution time. Sorting was considerably slower.

Both Mathematical Sum and XOR have linear time complexity, while Sorting has O(n log n) time complexity.

This comparison demonstrates that algorithms with the same time complexity can still have different practical execution times.
