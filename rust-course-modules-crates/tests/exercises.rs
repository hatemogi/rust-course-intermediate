#[test]
fn reexports_model_types_at_crate_root() {
    use rust_course_modules_crates::{Task, TaskStatus};

    let task = Task::new(1, "문서 작성", TaskStatus::Pending);
    assert_eq!(task.id(), 1);
    assert_eq!(task.title(), "문서 작성");
    assert_eq!(task.status(), TaskStatus::Pending);
}

#[test]
fn reexports_parser_function_at_crate_root() {
    use rust_course_modules_crates::{TaskStatus, parse_task};

    let task = parse_task(" 2 | 테스트 | done ").unwrap();
    assert_eq!(task.id(), 2);
    assert_eq!(task.title(), "테스트");
    assert_eq!(task.status(), TaskStatus::Done);
}

#[test]
fn reexports_parser_error_at_crate_root() {
    use rust_course_modules_crates::{ParseTaskError, parse_task};

    assert_eq!(parse_task("1|title"), Err(ParseTaskError::InvalidFormat));
    assert_eq!(parse_task("x|title|done"), Err(ParseTaskError::InvalidId));
    assert_eq!(parse_task("1| |done"), Err(ParseTaskError::EmptyTitle));
    assert_eq!(
        parse_task("1|title|unknown"),
        Err(ParseTaskError::InvalidStatus)
    );
}

#[test]
fn reexports_report_function_at_crate_root() {
    use rust_course_modules_crates::{Task, TaskStatus, render_report};

    let tasks = [
        Task::new(1, "첫째", TaskStatus::Done),
        Task::new(2, "둘째", TaskStatus::Pending),
        Task::new(3, "셋째", TaskStatus::Done),
    ];
    assert_eq!(render_report(&tasks), "완료 2개 / 대기 1개");
}
