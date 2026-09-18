use rust_course_patterns::{Command, Event, describe_event, parse_command};

fn command() {
    assert_eq!(
        parse_command(&["start", "backup"]),
        Some(Command::Start { job: "backup" })
    );
    assert_eq!(parse_command(&["unknown"]), None);
}

fn event() {
    assert_eq!(
        describe_event(&Event::Progress {
            job: String::from("backup"),
            percent: 100,
        }),
        "backup 작업 완료"
    );
}

fn main() {
    command();
    event();
}
