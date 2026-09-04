fn greet(name: &str) -> String {
    format!("안녕하세요, {name}!")
}

fn main() {
    // ANCHOR: types
    let borrowed: &str = "러스트";
    let owned: String = String::from("문자열");
    assert_eq!(borrowed, "러스트");
    assert_eq!(owned, "문자열");
    // ANCHOR_END: types

    // ANCHOR: borrow
    let language = String::from("Rust");
    let view: &str = &language;
    assert_eq!(greet(view), "안녕하세요, Rust!");
    assert_eq!(language, "Rust");
    // ANCHOR_END: borrow

    // ANCHOR: string_reference
    let course = String::from("Rust 문자열");
    let string_reference: &String = &course;
    let whole_slice: &str = string_reference;
    let partial_slice: &str = &course[..4];

    assert_eq!(greet(string_reference), "안녕하세요, Rust 문자열!");
    assert_eq!(whole_slice, "Rust 문자열");
    assert_eq!(partial_slice, "Rust");
    // ANCHOR_END: string_reference
}
