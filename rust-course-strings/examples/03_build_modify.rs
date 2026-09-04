fn main() {
    // ANCHOR: create
    let empty = String::new();
    assert!(empty.is_empty());

    let buffer = String::with_capacity(32);
    assert!(buffer.is_empty());
    assert!(buffer.capacity() >= 32);
    // ANCHOR_END: create

    // ANCHOR: from_str
    let from: String = String::from("Rust");
    let owned: String = "Rust".to_owned();

    assert_eq!(from, "Rust");
    assert_eq!(owned, "Rust");
    assert_eq!(from, owned);
    // ANCHOR_END: from_str

    // ANCHOR: modify
    let mut message = String::from("Rust");
    message.push(' ');
    message.push_str("문자열");
    assert_eq!(message, "Rust 문자열");
    // ANCHOR_END: modify

    // ANCHOR: replace
    let original = String::from("Rust는 빠르고 Rust는 안전합니다.");
    let replaced = original.replace("Rust", "러스트");

    assert_eq!(replaced, "러스트는 빠르고 러스트는 안전합니다.");
    assert_eq!(original, "Rust는 빠르고 Rust는 안전합니다.");
    // ANCHOR_END: replace

    // ANCHOR: add
    let left = String::from("Rust");
    let right = String::from("소유권");
    let combined = left + ": " + &right;

    assert_eq!(combined, "Rust: 소유권");
    assert_eq!(right, "소유권");
    // assert_eq!(left, "Rust"); // 컴파일 오류: left는 + 연산으로 이동했습니다.
    // ANCHOR_END: add

    // ANCHOR: format
    let language = String::from("Rust");
    let topic = String::from("소유권");
    let title = format!("{language}: {topic}");
    assert_eq!(title, "Rust: 소유권");
    assert_eq!(language, "Rust");
    // ANCHOR_END: format
}
