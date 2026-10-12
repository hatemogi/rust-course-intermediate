#[allow(clippy::question_mark, clippy::manual_map)] // ?를 쓰기 전의 분기 과정을 보여 줍니다.
fn half_sum_with_match(left: Option<i32>, right: Option<i32>) -> Option<i32> {
    let left = match left {
        Some(value) => value,
        None => return None,
    };
    let right = match right {
        Some(value) => value,
        None => return None,
    };
    match left.checked_add(right) {
        Some(sum) => Some(sum / 2),
        None => None,
    }
}

fn checked_average(left: Option<i32>, right: Option<i32>) -> Option<i32> {
    let left = left?;
    let right = right?;
    Some(left.checked_add(right)? / 2)
}

fn main() {
    for (left, right) in [
        (Some(10), Some(20)),
        (Some(10), None),
        (None, Some(20)),
        (Some(i32::MAX), Some(1)),
    ] {
        assert_eq!(
            checked_average(left, right),
            half_sum_with_match(left, right)
        );
    }
    assert_eq!(checked_average(Some(10), Some(20)), Some(15));
    assert_eq!(checked_average(Some(i32::MAX), Some(1)), None);
}
