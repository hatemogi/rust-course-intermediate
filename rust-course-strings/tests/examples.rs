use rust_course_strings::{
    initials, join_nonempty, normalize_spaces, redact_ascii_case_insensitive, safe_prefix,
    split_setting,
};

#[test]
fn normalizes_whitespace() {
    assert_eq!(
        normalize_spaces("  Rust   문자열\n강의 "),
        "Rust 문자열 강의"
    );
    assert_eq!(normalize_spaces(""), "");
}

#[test]
fn collects_initials() {
    assert_eq!(initials("Rust Language Course"), "RLC");
    assert_eq!(initials("러스트 문자열 강의"), "러문강");
}

#[test]
fn takes_prefix_without_breaking_utf8() {
    assert_eq!(safe_prefix("한글 Rust", 2), "한글");
    assert_eq!(safe_prefix("Rust", 10), "Rust");
}

#[test]
fn splits_and_trims_setting() {
    assert_eq!(split_setting(" PORT = 8080 "), Some(("PORT", "8080")));
    assert_eq!(split_setting("PORT"), None);
}

#[test]
fn joins_only_nonempty_values() {
    assert_eq!(join_nonempty(&["a", "", "b"], ","), "a,b");
    assert_eq!(join_nonempty(&[], ","), "");
}

#[test]
fn redacts_ascii_text_without_changing_other_text() {
    assert_eq!(
        redact_ascii_case_insensitive("Token=SECRET, secret", "secret"),
        "Token=******, ******"
    );
    assert_eq!(redact_ascii_case_insensitive("한글", ""), "한글");
}
