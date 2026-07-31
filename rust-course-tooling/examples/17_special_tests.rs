use std::{error::Error, num::ParseIntError};

fn parse_port(input: &str) -> Result<u16, ParseIntError> {
    input.parse()
}

fn first(values: &[i32]) -> i32 {
    values[0]
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(parse_port("8080")?, 8080);
    assert_eq!(first(&[10, 20]), 10);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{first, parse_port};
    use std::error::Error;

    #[test]
    fn parses_a_port() -> Result<(), Box<dyn Error>> {
        assert_eq!(parse_port("443")?, 443);
        Ok(())
    }

    #[test]
    fn rejects_a_port_with_letters() {
        let error = "http"
            .parse::<u16>()
            .expect_err("숫자가 아닌 포트는 거부해야 합니다");

        assert_eq!(error.kind(), &std::num::IntErrorKind::InvalidDigit);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn panics_when_the_slice_is_empty() {
        first(&[]);
    }

    #[test]
    #[ignore = "큰 입력을 직접 확인할 때만 실행합니다"]
    fn checks_many_ports() {
        for port in 0..=u16::MAX {
            assert_eq!(parse_port(&port.to_string()).unwrap(), port);
        }
    }
}
