use unicode_normalization::UnicodeNormalization;

fn main() {
    // ANCHOR: representations
    let composed = "한";
    let decomposed = "\u{1112}\u{1161}\u{11ab}";

    assert_eq!(composed.chars().count(), 1);
    assert_eq!(decomposed.chars().count(), 3);
    assert_ne!(composed, decomposed);
    // ANCHOR_END: representations

    // ANCHOR: nfc
    let decomposed = "\u{1112}\u{1161}\u{11ab}";
    let normalized = decomposed.nfc().collect::<String>();

    assert_eq!(normalized, "한");
    // ANCHOR_END: nfc

    // ANCHOR: nfd
    let composed = "한";
    let decomposed = composed.nfd().collect::<String>();

    assert_eq!(decomposed, "\u{1112}\u{1161}\u{11ab}");
    // ANCHOR_END: nfd

    // ANCHOR: compare
    let stored = "한글";
    let query = "\u{1112}\u{1161}\u{11ab}";
    assert!(!stored.contains(query));

    let normalized_query = query.nfc().collect::<String>();
    assert!(stored.contains(normalized_query.as_str()));
    // ANCHOR_END: compare
}
