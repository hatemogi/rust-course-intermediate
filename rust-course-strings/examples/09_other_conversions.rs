// ANCHOR: log_level_type
#[derive(Debug, PartialEq)]
enum LogLevel {
    Info,
    Warning,
    Error,
}

impl std::str::FromStr for LogLevel {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "info" => Ok(Self::Info),
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            _ => Err("알 수 없는 로그 수준"),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        };
        formatter.write_str(text)
    }
}
// ANCHOR_END: log_level_type

fn main() {
    // ANCHOR: boolean
    let enabled = "true".parse::<bool>().expect("유효한 불리언 값");
    assert!(enabled);
    assert_eq!(enabled.to_string(), "true");
    // ANCHOR_END: boolean

    // ANCHOR: character
    let letter = "한".parse::<char>().expect("문자 하나");
    assert_eq!(letter, '한');
    assert_eq!(letter.to_string(), "한");
    assert!("한글".parse::<char>().is_err());
    // ANCHOR_END: character

    // ANCHOR: network
    let ip = "127.0.0.1"
        .parse::<std::net::IpAddr>()
        .expect("유효한 IP 주소");
    let server = "127.0.0.1:8080"
        .parse::<std::net::SocketAddr>()
        .expect("유효한 소켓 주소");

    assert_eq!(ip.to_string(), "127.0.0.1");
    assert_eq!(server.to_string(), "127.0.0.1:8080");
    // ANCHOR_END: network

    // ANCHOR: log_level_usage
    let level = "warning".parse::<LogLevel>().expect("유효한 로그 수준");
    assert_eq!(level, LogLevel::Warning);
    assert_eq!(level.to_string(), "warning");
    // ANCHOR_END: log_level_usage
}
