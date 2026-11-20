pub fn find_missing_xor(n: u32, numbers: &[u32]) -> u32 {
    let mut result = 0;

    for i in 1..=n {
        result ^= i;
    }

    for &number in numbers {
        result ^= number;
    }

    result
}