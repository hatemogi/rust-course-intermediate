fn category(value: i32) -> &'static str {
    match value {
        ..0 => "음수",
        0 => "영",
        1..10 => "한 자리 양수",
        10..=99 => "두 자리 양수",
        _ => "세 자리 이상 양수",
    }
}

fn answer(input: &str) -> &'static str {
    match input {
        "y" | "Y" | "yes" | "YES" => "확인",
        "n" | "N" | "no" | "NO" => "취소",
        _ => "알 수 없음",
    }
}

fn is_confirmed(input: &str) -> bool {
    matches!(input, "y" | "Y" | "yes" | "YES")
}

fn main() {
    assert_eq!(category(-3), "음수");
    assert_eq!(category(7), "한 자리 양수");
    assert_eq!(category(42), "두 자리 양수");
    assert_eq!(category(100), "세 자리 이상 양수");
    assert_eq!(answer("yes"), "확인");
    assert_eq!(answer("N"), "취소");
    assert!(is_confirmed("yes"));
    assert!(!is_confirmed("no"));
}
