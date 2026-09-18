#![allow(clippy::let_and_return)] // 퀴즈에서 묻는 변수 이름을 유지하고 결과는 main에서 검사합니다.

#[allow(clippy::redundant_pattern_matching)]
fn underscore() -> Option<String> {
    let kept = Some(String::from("Rust"));
    if let Some(_) = kept {}
    let moved = Some(String::from("Rust"));
    if let Some(_text) = moved {}

    kept
}

fn boundary() -> &'static str {
    let label = match 10 {
        1..10 => "가",
        10..=99 => "나",
        _ => "다",
    };

    label
}

fn guard() -> &'static str {
    let limit = 5;
    let label = match Some(3) {
        Some(x) if x > limit => "초과",
        Some(_) => "범위 안",
        None => "없음",
    };

    label
}

fn reference() -> (usize, Option<String>) {
    let message = Some(String::from("Rust"));
    let length = match &message {
        Some(text) => text.len(),
        None => 0,
    };

    (length, message)
}

fn main() {
    assert_eq!(underscore().as_deref(), Some("Rust"));
    assert_eq!(boundary(), "나");
    assert_eq!(guard(), "범위 안");
    let (length, message) = reference();
    assert_eq!(length, 4);
    assert_eq!(message.as_deref(), Some("Rust"));
}
