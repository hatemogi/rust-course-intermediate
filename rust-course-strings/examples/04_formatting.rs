fn main() {
    // ANCHOR: values
    let language = "Rust";
    let edition = 2024;

    assert_eq!(format!("{} {}", language, edition), "Rust 2024");
    assert_eq!(format!("{language} {edition}"), "Rust 2024");
    assert_eq!(
        format!("{name} 강의: {name} {edition}", name = language),
        "Rust 강의: Rust 2024"
    );
    // ANCHOR_END: values

    // ANCHOR: alignment
    let word = "Rust";

    assert_eq!(format!("|{word:<8}|"), "|Rust    |");
    assert_eq!(format!("|{word:>8}|"), "|    Rust|");
    assert_eq!(format!("|{word:-^10}|"), "|---Rust---|");

    let width = 6;
    assert_eq!(format!("|{word:>width$}|"), "|  Rust|");
    // ANCHOR_END: alignment

    // ANCHOR: integers
    let number = 42;

    assert_eq!(format!("{number:b}"), "101010");
    assert_eq!(format!("{number:o}"), "52");
    assert_eq!(format!("{number:x}"), "2a");
    assert_eq!(format!("{number:X}"), "2A");
    assert_eq!(format!("{number:#x}"), "0x2a");
    assert_eq!(format!("{number:+}"), "+42");
    assert_eq!(format!("{number:05}"), "00042");
    // ANCHOR_END: integers

    // ANCHOR: floats
    let value = std::f64::consts::PI;

    assert_eq!(format!("{value:.2}"), "3.14");
    assert_eq!(format!("{value:8.2}"), "    3.14");

    let precision = 4;
    assert_eq!(format!("{value:.precision$}"), "3.1416");
    // ANCHOR_END: floats

    // ANCHOR: debug
    let scores = [82, 95, 100];

    assert_eq!(format!("{scores:?}"), "[82, 95, 100]");
    assert_eq!(format!("{scores:#?}"), "[\n    82,\n    95,\n    100,\n]");
    // ANCHOR_END: debug
}
