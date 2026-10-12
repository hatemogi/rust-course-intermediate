fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("서준")),
        2 => Some(String::from("지우")),
        _ => None,
    }
}

fn match_option(id: u32) -> String {
    match find_user(id) {
        Some(name) => format!("사용자: {name}"),
        None => String::from("사용자를 찾지 못했습니다"),
    }
}

fn if_let_option(id: u32) {
    if let Some(name) = find_user(id) {
        println!("{name}님, 안녕하세요!");
    }
}

fn main() {
    assert_eq!(find_user(1), Some(String::from("서준")));
    assert_eq!(find_user(99), None);
    assert_eq!(match_option(1), "사용자: 서준");
    assert_eq!(match_option(99), "사용자를 찾지 못했습니다");
    if_let_option(2);
    if_let_option(99);
}
