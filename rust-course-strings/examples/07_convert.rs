fn main() {
    // ANCHOR: number_to_string
    let count = 42;
    let count_text = count.to_string();
    let padded = format!("{count:04}");

    assert_eq!(count_text, "42");
    assert_eq!(padded, "0042");
    // ANCHOR_END: number_to_string

    // ANCHOR: string_to_number
    let port: u16 = "8080".parse().expect("유효한 포트 번호");
    let ratio = "0.75".parse::<f64>().expect("유효한 비율");

    assert_eq!(port, 8080);
    assert_eq!(ratio, 0.75);
    // ANCHOR_END: string_to_number

    // ANCHOR: parse_error
    let invalid_number = "12개".parse::<u32>();
    assert!(invalid_number.is_err());
    // ANCHOR_END: parse_error
}
