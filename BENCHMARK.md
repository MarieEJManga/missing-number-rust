# Missing Number – Benchmark Report

## 1. Benchmark Setup

Three algorithms were implemented to solve the Missing Number problem:

- Mathematical Sum
- Sorting
- XOR (Bitwise Operation)

The benchmark was performed using:

- Input size: 1,000,000 numbers
- Iterations: 20
- Build mode: Release
- Programming language: Rust

## 2. Benchmark Results

All three algorithms passed correctness checks.

| Algorithm | Average execution time |
|---|---:|
| Mathematical Sum | 0.998 ms |
| Sorting | 1.870 ms |
| XOR | 0.632 ms |

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

The XOR algorithm was the fastest in this benchmark, with an average execution time of 0.632 ms.

The Mathematical Sum algorithm took 0.998 ms, while the Sorting algorithm took 1.870 ms.

The XOR algorithm uses bitwise operations to cancel out matching numbers and identify the missing number. It does not need to calculate a large mathematical sum.

The Mathematical Sum algorithm uses a formula to calculate the expected sum and subtracts the actual sum.

The Sorting algorithm sorts the numbers before searching for the missing value, which requires more processing time.

XOR was approximately 1.58 times faster than Mathematical Sum and 2.96 times faster than Sorting in this benchmark.

Benchmark results can vary depending on the computer and system conditions.

## 5. Conclusion

All three algorithms correctly solve the Missing Number problem.

XOR performed best in this benchmark. Both XOR and Mathematical Sum have linear time complexity, while Sorting has O(n log n) time complexity.

This comparison demonstrates that algorithms with the same time complexity can still have different practical execution times.