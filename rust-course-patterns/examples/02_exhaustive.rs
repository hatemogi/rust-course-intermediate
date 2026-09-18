enum Status {
    Ready,
    Running,
    Done,
}

fn label(status: Status) -> &'static str {
    match status {
        Status::Ready => "준비",
        Status::Running => "실행 중",
        Status::Done => "완료",
    }
}

fn activity_label(status: Status) -> &'static str {
    match status {
        Status::Running => "실행 중",
        _ => "실행 중 아님",
    }
}

fn main() {
    assert_eq!(label(Status::Ready), "준비");
    assert_eq!(label(Status::Running), "실행 중");
    assert_eq!(label(Status::Done), "완료");

    assert_eq!(activity_label(Status::Running), "실행 중");
    assert_eq!(activity_label(Status::Ready), "실행 중 아님");
    assert_eq!(activity_label(Status::Done), "실행 중 아님");
}
