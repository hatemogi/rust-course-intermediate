fn contains_even(numbers: &[i32]) -> bool {
    numbers.iter().any(|number| number % 2 == 0)
}

fn main() {
    assert!(contains_even(&[1, 3, 4]));
    assert!(!contains_even(&[1, 3, 5]));
}
