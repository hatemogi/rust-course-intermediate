use rust_course_modules_crates::{Task, TaskStatus, parse_task, render_report};

fn main() {
    // ANCHOR: exercise
    let first = parse_task("1|문서 작성|done").expect("유효한 작업");
    let second = Task::new(2, "테스트 작성", TaskStatus::Pending);
    let report = render_report(&[first, second]);
    assert_eq!(report, "완료 1개 / 대기 1개");
    // ANCHOR_END: exercise
}
