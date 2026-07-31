fn contains_even(numbers: &[i32]) -> bool {
    numbers
        .iter()
        .map(|number| number % 2 == 0)
        .any(|is_even| is_even)
}

fn main() {
    assert!(contains_even(&[1, 3, 4]));
    assert!(!contains_even(&[1, 3, 5]));
}
