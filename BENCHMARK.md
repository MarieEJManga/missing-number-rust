# Missing Number – Benchmark Report

## 1. Benchmark Setup

Two algorithms were implemented to solve the Missing Number problem:

- Mathematical Sum
- Sorting

The benchmark was performed using:
- Input size: 1,000,000 numbers
- Iterations: 20
- Build mode: Release
- Programming language: Rust

## 2. Benchmark Results

| Algorithm | Average execution time |
|---|---:|
| Mathematical Sum | 1.364 ms |
| Sorting | 2.563 ms |

Both algorithms passed correctness checks.

## 3. Complexity Analysis

**Mathematical Sum**
- Time complexity: O(n)
- Auxiliary space complexity: O(1)

**Sorting**
- Time complexity: O(n log n)
- Additional space complexity: Depends on the sorting implementation.

## 4. Interpretation

The Mathematical Sum algorithm was faster than the Sorting algorithm in this benchmark.

It uses a mathematical formula to calculate the expected sum and subtracts the actual sum of the input numbers.

The Sorting algorithm first sorts the numbers and then searches for the missing value. Sorting requires more processing time.

Benchmark results can vary depending on the computer and system conditions.

## 5. Conclusion

Both algorithms correctly solve the Missing Number problem.

The Mathematical Sum algorithm performed better in this benchmark because it has linear time complexity, while the Sorting algorithm has O(n log n) time complexity.