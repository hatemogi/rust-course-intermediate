// ANCHOR: sum
fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}
// ANCHOR_END: sum

fn double(values: &mut [i32]) {
    for value in values {
        *value *= 2;
    }
}

// ANCHOR: parameter
fn compare_inputs() {
    let fixed = [1, 2, 3];
    let dynamic = vec![4, 5, 6];
    let vector_reference: &Vec<i32> = &dynamic;
    let slice_reference: &[i32] = vector_reference;

    assert_eq!(sum(&fixed), 6);
    assert_eq!(sum(&dynamic), 15);
    assert_eq!(sum(&dynamic[1..]), 11);
    assert_eq!(slice_reference, &[4, 5, 6]);
}
// ANCHOR_END: parameter

fn main() {
    // ANCHOR: range
    let scores = [70, 80, 90, 100];
    let middle: &[i32] = &scores[1..3];

    assert_eq!(middle, &[80, 90]);
    assert_eq!(scores, [70, 80, 90, 100]);
    // ANCHOR_END: range

    compare_inputs();

    // ANCHOR: mutable
    let mut values = [1, 2, 3, 4];
    double(&mut values[1..3]);

    assert_eq!(values, [1, 4, 6, 4]);
    // ANCHOR_END: mutable
}
