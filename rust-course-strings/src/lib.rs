//! Rust 문자열 강의의 종합 예제 함수들입니다.

// ANCHOR: normalize_spaces
/// 연속된 공백을 하나로 줄이고 앞뒤 공백을 제거합니다.
pub fn normalize_spaces(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}
// ANCHOR_END: normalize_spaces

// ANCHOR: initials
/// 각 단어의 첫 유니코드 스칼라 값을 이어 붙입니다.
pub fn initials(input: &str) -> String {
    input
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect()
}
// ANCHOR_END: initials

// ANCHOR: safe_prefix
/// 앞에서 최대 `count`개의 유니코드 스칼라 값을 새 문자열로 반환합니다.
pub fn safe_prefix(input: &str, count: usize) -> String {
    input.chars().take(count).collect()
}
// ANCHOR_END: safe_prefix

// ANCHOR: split_setting
/// `KEY=VALUE` 형식을 빌린 문자열 두 개로 나눕니다.
pub fn split_setting(input: &str) -> Option<(&str, &str)> {
    let (key, value) = input.split_once('=')?;
    Some((key.trim(), value.trim()))
}
// ANCHOR_END: split_setting

// ANCHOR: join_nonempty
/// 비어 있지 않은 문자열만 구분자로 연결합니다.
pub fn join_nonempty(values: &[&str], separator: &str) -> String {
    values
        .iter()
        .copied()
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>()
        .join(separator)
}
// ANCHOR_END: join_nonempty

// ANCHOR: redact
/// 대소문자를 구분하지 않고 ASCII 단어를 가립니다.
pub fn redact_ascii_case_insensitive(input: &str, secret: &str) -> String {
    if secret.is_empty() {
        return input.to_owned();
    }

    let lower_input = input.to_ascii_lowercase();
    let lower_secret = secret.to_ascii_lowercase();
    let mut result = String::with_capacity(input.len());
    let mut start = 0;

    while let Some(relative) = lower_input[start..].find(&lower_secret) {
        let found = start + relative;
        result.push_str(&input[start..found]);
        result.push_str(&"*".repeat(secret.len()));
        start = found + secret.len();
    }
    result.push_str(&input[start..]);
    result
}
// ANCHOR_END: redact
