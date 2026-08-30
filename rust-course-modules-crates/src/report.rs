use crate::model::{Task, TaskStatus};

pub fn render_report(tasks: &[Task]) -> String {
    let done = tasks
        .iter()
        .filter(|task| task.status() == TaskStatus::Done)
        .count();
    let pending = tasks.len() - done;
    format!("완료 {done}개 / 대기 {pending}개")
}
