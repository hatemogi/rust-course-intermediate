fn listen_address(port: Option<u16>) -> Result<String, &'static str> {
    let port_result: Result<u16, &'static str> = port.ok_or("포트를 설정해야 합니다");
    let port = port_result?;
    Ok(format!("127.0.0.1:{port}"))
}

fn parse_listen_address(input: &str) -> Result<String, String> {
    let port_result = input.parse::<u16>();
    let port_result: Result<u16, String> =
        port_result.map_err(|error| format!("포트 설정 '{input}'을 읽을 수 없습니다: {error}"));
    let port = port_result?;
    Ok(format!("127.0.0.1:{port}"))
}

#[derive(Debug)]
enum AddressError {
    InvalidPort(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for AddressError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::InvalidPort(error)
    }
}

fn parse_address_with_from(input: &str) -> Result<String, AddressError> {
    let port = input.parse::<u16>()?;
    Ok(format!("127.0.0.1:{port}"))
}

fn address_if_valid(input: &str) -> Option<String> {
    let port_result = input.parse::<u16>();
    let port_option: Option<u16> = port_result.ok();
    let port = port_option?;
    Some(format!("127.0.0.1:{port}"))
}

fn address_if_valid_chained(input: &str) -> Option<String> {
    let port = input.parse::<u16>().ok()?;
    Some(format!("127.0.0.1:{port}"))
}

fn main() {
    for input in ["8080", "rust", "70000"] {
        assert_eq!(address_if_valid_chained(input), address_if_valid(input));
    }
    assert_eq!(parse_address_with_from("8080").unwrap(), "127.0.0.1:8080");
    for (input, expected_kind) in [
        ("rust", std::num::IntErrorKind::InvalidDigit),
        ("70000", std::num::IntErrorKind::PosOverflow),
    ] {
        let AddressError::InvalidPort(error) = parse_address_with_from(input).unwrap_err();
        assert_eq!(error.kind(), &expected_kind);
    }
    assert_eq!(
        parse_listen_address("8080"),
        Ok("127.0.0.1:8080".to_owned())
    );
    let error = parse_listen_address("rust").unwrap_err();
    assert!(error.starts_with("포트 설정 'rust'을 읽을 수 없습니다:"));
    assert!(parse_listen_address("70000").is_err());
    assert_eq!(address_if_valid("8080"), Some("127.0.0.1:8080".to_owned()));
    assert_eq!(address_if_valid("rust"), None);
    assert_eq!(address_if_valid("70000"), None);
    assert_eq!(listen_address(Some(8080)), Ok("127.0.0.1:8080".to_owned()));
    assert_eq!(listen_address(None), Err("포트를 설정해야 합니다"));
}
