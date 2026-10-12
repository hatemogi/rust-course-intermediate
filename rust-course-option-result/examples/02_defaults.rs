#![allow(clippy::unnecessary_literal_unwrap)] // 계산 시점을 비교하려고 Some과 None을 직접 지정합니다.

fn unwrap_option() {
    let port = Some(8080);
    assert_eq!(port.unwrap(), 8080);
    assert_eq!(port.expect("포트가 설정되어 있어야 합니다"), 8080);

    // None::<u16>.unwrap(); // 실행하면 패닉이 발생합니다.
}

fn defaults() {
    let mut calls = 0;
    let mut default_port = || {
        calls += 1;
        3000
    };
    assert_eq!(Some(8080).unwrap_or(default_port()), 8080);
    assert_eq!(calls, 1); // Some이어도 기본값을 계산했습니다.

    let mut calls = 0;
    let mut default_port = || {
        calls += 1;
        3000
    };
    assert_eq!(Some(8080).unwrap_or_else(&mut default_port), 8080);
    assert_eq!(None::<u16>.unwrap_or_else(&mut default_port), 3000);
    assert_eq!(calls, 1); // None일 때만 계산했습니다.
}

fn main() {
    unwrap_option();
    defaults();
}
