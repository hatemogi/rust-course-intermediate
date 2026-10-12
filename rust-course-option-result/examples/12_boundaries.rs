use std::{num::ParseIntError, process::ExitCode};

fn load_port(input: &str) -> Result<u16, ParseIntError> {
    input.parse()
}

fn run(input: &str) -> ExitCode {
    match load_port(input) {
        Ok(port) => {
            println!("설정한 포트: {port}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("포트 설정을 확인해 주세요: {error}");
            ExitCode::from(2)
        }
    }
}

fn main() -> ExitCode {
    let input = std::env::args().nth(1).unwrap_or_else(|| "8080".to_owned());
    run(&input)
}
