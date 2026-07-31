use rust_course_tooling::{count_words, normalize_names};

fn main() {
    let names = normalize_names(&[" Ferris ", " RUST ", ""]);
    let counts = count_words("rust cargo rust clippy");

    println!("정돈한 이름: {names:?}");
    println!("단어별 횟수: {counts:?}");
}
