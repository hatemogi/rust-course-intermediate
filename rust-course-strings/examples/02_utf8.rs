fn main() {
    // ANCHOR: lengths
    let text = "Rust 한글";
    assert_eq!(text.len(), 11); // ①
    assert_eq!(text.chars().count(), 7); // ②
    let encoded: Vec<u8> = text.bytes().collect(); // ③
    assert_eq!(encoded.len(), 11);
    // ANCHOR_END: lengths

    // ANCHOR: byte_conversion
    let text = String::from("Rust 한글");
    let borrowed_bytes: &[u8] = text.as_bytes();
    assert_eq!(borrowed_bytes.len(), 11);

    let owned_bytes: Vec<u8> = text.into_bytes();
    let restored = String::from_utf8(owned_bytes).expect("유효한 UTF-8 바이트");
    assert_eq!(restored, "Rust 한글");
    // ANCHOR_END: byte_conversion

    // ANCHOR: safe_slice
    let word = "한글";
    assert_eq!(word.get(0..3), Some("한"));
    assert_eq!(word.get(0..1), None);
    assert_eq!(word.chars().next(), Some('한'));
    // ANCHOR_END: safe_slice
}
