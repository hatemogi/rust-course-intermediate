fn main() {
    // ANCHOR: contains
    let course = "Rust 문자열 강의.md";

    assert!(course.contains("문자열"));
    assert!(course.starts_with("Rust"));
    assert!(course.ends_with(".md"));
    // ANCHOR_END: contains

    // ANCHOR: position
    let text = "Rust와 Rust 문자열";

    assert_eq!(text.find("Rust"), Some(0));
    assert_eq!(text.rfind("Rust"), Some(8));
    assert_eq!(text.find("Python"), None);
    // ANCHOR_END: position
}
