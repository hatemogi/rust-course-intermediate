fn required_port(input: Option<&str>) -> Result<u16, &'static str> {
    let text = input.ok_or("포트를 설정해야 합니다")?;
    text.parse()
        .map_err(|_| "포트 번호는 숫자로 입력해야 합니다")
}

fn parse_port(input: &str) -> Option<u16> {
    input.parse::<u16>().ok()
}

fn discard_error() {
    assert_eq!(parse_port("8080"), Some(8080));
    assert_eq!(parse_port("rust"), None);
    assert_eq!(parse_port("65536"), None);
}

fn main() {
    discard_error();
    assert_eq!(required_port(Some("8080")), Ok(8080));
    assert!(required_port(None).is_err());
    assert!(required_port(Some("rust")).is_err());
}
