use std::{fmt, fs, io};

#[derive(Debug)]
struct ReadConfigError {
    path: String,
    source: io::Error,
}

impl fmt::Display for ReadConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} 설정 파일을 읽지 못했습니다", self.path)
    }
}

impl std::error::Error for ReadConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

fn read_config(path: &str) -> Result<String, ReadConfigError> {
    fs::read_to_string(path).map_err(|source| ReadConfigError {
        path: path.to_owned(),
        source,
    })
}

fn main() {
    let error = read_config("없는-설정.toml").unwrap_err();
    assert!(error.to_string().contains("없는-설정.toml"));
}
