use rust_course_patterns::{Command, Event, describe_event, parse_command};

#[test]
fn parses_known_commands() {
    assert_eq!(
        parse_command(&["start", "backup"]),
        Some(Command::Start { job: "backup" })
    );
    assert_eq!(parse_command(&["stop"]), Some(Command::Stop));
    assert_eq!(parse_command(&["status"]), Some(Command::Status));
}

#[test]
fn rejects_unknown_command_shapes() {
    assert_eq!(parse_command(&[]), None);
    assert_eq!(parse_command(&["start"]), None);
    assert_eq!(parse_command(&["stop", "extra"]), None);
    assert_eq!(parse_command(&["unknown"]), None);
}

#[test]
fn describes_each_event_case() {
    assert_eq!(
        describe_event(&Event::Started {
            job: String::from("backup")
        }),
        "backup 작업 시작"
    );
    assert_eq!(
        describe_event(&Event::Progress {
            job: String::from("backup"),
            percent: 40
        }),
        "backup 작업 40%"
    );
    assert_eq!(
        describe_event(&Event::Progress {
            job: String::from("backup"),
            percent: 100
        }),
        "backup 작업 완료"
    );
    assert_eq!(
        describe_event(&Event::Progress {
            job: String::from("backup"),
            percent: 101
        }),
        "backup 작업의 잘못된 진행률: 101"
    );
    assert_eq!(
        describe_event(&Event::Failed {
            job: String::from("backup")
        }),
        "backup 작업 실패"
    );
}
