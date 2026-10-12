fn parse_port(input: &str) -> Result<u16, String> {
    input
        .parse::<u16>()
        .map_err(|error| format!("포트를 읽지 못했습니다: {error}"))
}

fn require_nonzero(port: u16) -> Result<u16, String> {
    if port == 0 {
        Err("이 설정에서는 포트 0을 사용할 수 없습니다".to_owned())
    } else {
        Ok(port)
    }
}

fn combinators() {
    assert_eq!(
        parse_port("8080").map(|port| format!("포트: {port}")),
        Ok("포트: 8080".to_owned())
    );
    assert!(
        parse_port("rust")
            .unwrap_err()
            .contains("포트를 읽지 못했습니다")
    );
    assert_eq!(parse_port("8080").and_then(require_nonzero), Ok(8080));
    assert!(parse_port("0").and_then(require_nonzero).is_err());
    assert!(parse_port("rust").and_then(require_nonzero).is_err());
}

fn main() {
    combinators();
}
