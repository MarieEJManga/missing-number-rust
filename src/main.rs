mod math;
mod sorting;
mod xor;

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut numbers = input.split_whitespace();

    let n: u64 = numbers.next().unwrap().parse().unwrap();

    let values: Vec<u64> = numbers
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let math_result = math::find_missing_math(n, &values);

    let sorting_result = sorting::find_missing_sorting(n, values.clone());

    let xor_values: Vec<u32> = values.iter().map(|&x| x as u32).collect();

    let xor_result = xor::find_missing_xor(n as u32, &xor_values);

    assert_eq!(math_result, sorting_result);
    assert_eq!(math_result as u32, xor_result);

    println!("{}", math_result);
}