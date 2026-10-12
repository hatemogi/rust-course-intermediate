use std::num::ParseIntError;

#[derive(Debug)]
enum ConfigError {
    MissingValue,
    InvalidPort(ParseIntError),
}

fn parse_port(input: Option<&str>) -> Result<u16, ConfigError> {
    let text = input.ok_or(ConfigError::MissingValue)?;
    text.parse().map_err(ConfigError::InvalidPort)
}

fn main() {
    assert_eq!(parse_port(Some("8080")).unwrap(), 8080);
    assert!(matches!(parse_port(None), Err(ConfigError::MissingValue)));
    match parse_port(Some("rust")) {
        Err(ConfigError::InvalidPort(source)) => {
            assert!(matches!(
                source.kind(),
                std::num::IntErrorKind::InvalidDigit
            ));
        }
        other => panic!("숫자가 아닌 값을 넣었을 때의 결과가 예상과 다릅니다: {other:?}"),
    }
}
