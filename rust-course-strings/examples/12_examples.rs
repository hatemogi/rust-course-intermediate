use rust_course_strings::{
    initials, join_nonempty, normalize_spaces, redact_ascii_case_insensitive, safe_prefix,
    split_setting,
};

fn main() {
    // ANCHOR: example
    assert_eq!(normalize_spaces("  Rust   문자열 "), "Rust 문자열");
    assert_eq!(initials("Rust Language Course"), "RLC");
    assert_eq!(safe_prefix("한글 Rust", 3), "한글 ");
    assert_eq!(split_setting(" PORT = 8080 "), Some(("PORT", "8080")));
    assert_eq!(join_nonempty(&["Rust", "", "문자열"], " "), "Rust 문자열");
    assert_eq!(
        redact_ascii_case_insensitive("Token=SECRET", "secret"),
        "Token=******"
    );
    // ANCHOR_END: example
}
