pub fn find_missing_sorting(n: u64, mut numbers: Vec<u64>) -> u64 {
    numbers.sort_unstable();

    for i in 0..n as usize {
        if i >= numbers.len() || numbers[i] != (i + 1) as u64 {
            return (i + 1) as u64;
        }
    }

    n
}