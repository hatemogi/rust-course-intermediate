fn divide(left: i32, right: i32) -> Result<i32, &'static str> {
    if right == 0 {
        Err("0으로 나눌 수 없습니다")
    } else if left == i32::MIN && right == -1 {
        Err("계산 결과가 i32 범위를 벗어납니다")
    } else {
        Ok(left / right)
    }
}

#[allow(clippy::unnecessary_literal_unwrap)] // Ok에서 값을 꺼내는 방법을 보여 줍니다.
fn unwrap_result() {
    let result: Result<i32, &str> = Ok(4);
    assert_eq!(result.unwrap(), 4);
    assert_eq!(result.expect("나눗셈이 성공해야 합니다"), 4);

    // Err::<i32, &str>("0으로 나눌 수 없습니다").unwrap(); // 실행하면 패닉이 발생합니다.
}

fn recover() {
    let input = "rust";
    let port = match input.parse::<u16>() {
        Ok(port) => port,
        Err(error) => {
            eprintln!("'{input}'을 포트 번호로 바꿀 수 없습니다: {error}. 기본값을 사용합니다.");
            3000
        }
    };
    assert_eq!(port, 3000);
}

fn main() {
    assert_eq!(divide(12, 3), Ok(4));
    assert_eq!(divide(12, 0), Err("0으로 나눌 수 없습니다"));
    assert_eq!(
        divide(i32::MIN, -1),
        Err("계산 결과가 i32 범위를 벗어납니다")
    );
    unwrap_result();
    recover();
}
