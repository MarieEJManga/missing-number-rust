use missing_numbers::math::find_missing_math;
use missing_numbers::sorting::find_missing_sorting;

#[test]
fn test_missing_number_at_beginning() {
    let numbers = vec![2, 3, 4, 5];

    assert_eq!(find_missing_math(5, &numbers), 1);
    assert_eq!(find_missing_sorting(5, numbers), 1);
}

#[test]
fn test_missing_number_in_middle() {
    let numbers = vec![1, 2, 3, 5, 6];

    assert_eq!(find_missing_math(6, &numbers), 4);
    assert_eq!(find_missing_sorting(6, numbers), 4);
}

#[test]
fn test_missing_number_at_end() {
    let numbers = vec![1, 2, 3, 4];

    assert_eq!(find_missing_math(5, &numbers), 5);
    assert_eq!(find_missing_sorting(5, numbers), 5);
}

#[test]
fn test_both_algorithms_agree() {
    let numbers = vec![1, 2, 4, 5, 6, 7];

    let math_result = find_missing_math(7, &numbers);
    let sorting_result = find_missing_sorting(7, numbers);

    assert_eq!(math_result, sorting_result);
    assert_eq!(math_result, 3);
}