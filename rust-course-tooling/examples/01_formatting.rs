fn greeting(name: &str) -> String {
    format!("안녕하세요, {name}님")
}

fn main() {
    assert_eq!(greeting("Ferris"), "안녕하세요, Ferris님");
}
