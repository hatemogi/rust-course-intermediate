fn main() {
    // ANCHOR: split
    let parts: Vec<_> = "빨강,초록,,파랑".split(',').collect();
    assert_eq!(parts, ["빨강", "초록", "", "파랑"]);

    let colors: Vec<_> = "빨강,초록,,파랑"
        .split(',')
        .filter(|part| !part.is_empty())
        .collect();
    assert_eq!(colors, ["빨강", "초록", "파랑"]);
    // ANCHOR_END: split

    // ANCHOR: split_once
    let setting = "PORT=8080";
    let (key, value) = setting.split_once('=').expect("KEY=VALUE 형식");
    assert_eq!((key, value), ("PORT", "8080"));
    // ANCHOR_END: split_once

    // ANCHOR: split_whitespace
    let words: Vec<_> = "  Rust   문자열  ".split_whitespace().collect();
    assert_eq!(words, ["Rust", "문자열"]);
    // ANCHOR_END: split_whitespace

    // ANCHOR: join
    let topics = ["소유권", "슬라이스", "문자열"];
    let course_title = topics.join(" · ");

    assert_eq!(course_title, "소유권 · 슬라이스 · 문자열");
    assert_eq!(topics[0], "소유권");
    // ANCHOR_END: join

    // ANCHOR: number_join
    let numbers = [10, 20, 30];
    let number_strings: Vec<_> = numbers.iter().map(|number| number.to_string()).collect();
    let text = number_strings.join(", ");

    assert_eq!(text, "10, 20, 30");
    // ANCHOR_END: number_join
}
