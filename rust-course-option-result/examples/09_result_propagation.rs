use std::num::ParseIntError;

#[allow(clippy::question_mark)] // ?를 쓰기 전에 오류를 반환하는 과정을 보여 줍니다.
fn sum_with_match(left: &str, right: &str) -> Result<i64, ParseIntError> {
    let left = match left.parse::<i32>() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let right = match right.parse::<i32>() {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    Ok(left as i64 + right as i64)
}

fn sum(left: &str, right: &str) -> Result<i64, ParseIntError> {
    let left = left.parse::<i32>()?;
    let right = right.parse::<i32>()?;
    // i32 값 두 개의 합은 i64 범위 안에 들어갑니다.
    Ok(left as i64 + right as i64)
}

fn main() {
    for (left, right) in [
        ("10", "20"),
        ("10", "rust"),
        ("rust", "20"),
        ("2147483647", "1"),
    ] {
        assert_eq!(sum(left, right), sum_with_match(left, right));
    }
    assert_eq!(sum("10", "20"), Ok(30));
}
