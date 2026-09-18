//! Rust 패턴 문법 강의의 종합 실습 타입과 참고 구현입니다.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command<'a> {
    Start { job: &'a str },
    Stop,
    Status,
}

/// 슬라이스 패턴으로 명령 이름과 인자를 함께 확인합니다.
pub fn parse_command<'a>(args: &[&'a str]) -> Option<Command<'a>> {
    match args {
        ["start", job] => Some(Command::Start { job }),
        ["stop"] => Some(Command::Stop),
        ["status"] => Some(Command::Status),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Started { job: String },
    Progress { job: String, percent: u8 },
    Failed { job: String },
}

/// 열거형을 분해하고 구체적인 경우부터 작업 상태를 설명합니다.
pub fn describe_event(event: &Event) -> String {
    match event {
        Event::Started { job } => format!("{job} 작업 시작"),
        Event::Progress { job, percent: 100 } => format!("{job} 작업 완료"),
        Event::Progress {
            job,
            percent: percent @ 0..=99,
        } => format!("{job} 작업 {percent}%"),
        Event::Progress { job, percent } => format!("{job} 작업의 잘못된 진행률: {percent}"),
        Event::Failed { job } => format!("{job} 작업 실패"),
    }
}
