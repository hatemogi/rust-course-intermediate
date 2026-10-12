#[allow(clippy::manual_map)] // map을 쓰기 전의 코드를 비교합니다.
fn double_with_match(input: Option<i32>) -> Option<i32> {
    match input {
        Some(value) => Some(value * 2),
        None => None,
    }
}

fn double_with_map(input: Option<i32>) -> Option<i32> {
    input.map(|value| value * 2)
}

fn transform() {
    let input = Some(10_i32);
    let nested = input.map(|value| value.checked_div(2));
    let flat = input.and_then(|value| value.checked_div(2));
    assert_eq!(nested, Some(Some(5)));
    assert_eq!(flat, Some(5));
    assert_eq!(input.and_then(|value| value.checked_div(0)), None);
}

fn alternatives() {
    assert_eq!(Some(80).filter(|port| *port >= 1024), None);
}

fn main() {
    assert_eq!(double_with_match(Some(21)), double_with_map(Some(21)));
    assert_eq!(double_with_match(None), double_with_map(None));
    for (value, divisor) in [(10_i32, 2), (10, 0), (i32::MIN, -1), (-10, 3)] {
        assert_eq!(
            checked_div_example(value, divisor),
            value.checked_div(divisor)
        );
    }
    transform();
    alternatives();
    fallback_checks();
}

// i32::checked_div의 동작을 이해하기 위한 학습용 구현입니다.
fn checked_div_example(value: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 || (value == i32::MIN && divisor == -1) {
        None
    } else {
        Some(value / divisor)
    }
}

#[allow(clippy::unnecessary_lazy_evaluations)] // or_else의 반환형을 보여 줍니다.
fn default_with_or_else(port: Option<u16>) -> Option<u16> {
    port.or_else(|| Some(3000))
}

fn port_from_settings(port: Option<u16>, settings: &str) -> Option<u16> {
    port.or_else(|| settings.parse::<u16>().ok())
}

fn fallback_checks() {
    assert_eq!(default_with_or_else(Some(8080)), Some(8080));
    assert_eq!(default_with_or_else(None), Some(3000));
    assert_eq!(port_from_settings(Some(8080), "4000"), Some(8080));
    assert_eq!(port_from_settings(None, "4000"), Some(4000));
    assert_eq!(port_from_settings(None, "잘못된 설정"), None);

    let mut calls = 0;
    for port in [Some(8080), None] {
        let _ = port.or_else(|| {
            calls += 1;
            Some(3000)
        });
    }
    assert_eq!(calls, 1);
}
