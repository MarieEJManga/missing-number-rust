use std::io::{self, Read};

fn find_missing_math(n: u64, numbers: &[u64]) -> u64 {
    let expected_sum = n * (n + 1) / 2;
    let actual_sum: u64 = numbers.iter().sum();

    expected_sum - actual_sum
}

fn find_missing_sorting(n: u64, mut numbers: Vec<u64>) -> u64 {
    numbers.sort_unstable();

    for i in 0..n as usize {
        if i >= numbers.len() || numbers[i] != (i + 1) as u64 {
            return (i + 1) as u64;
        }
    }

    n
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut numbers = input.split_whitespace();

    let n: u64 = numbers.next().unwrap().parse().unwrap();

    let values: Vec<u64> = numbers
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let math_result = find_missing_math(n, &values);
    let sorting_result = find_missing_sorting(n, values.clone());

    assert_eq!(math_result, sorting_result);

    println!("{}", math_result);
}