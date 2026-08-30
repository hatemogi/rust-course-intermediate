//! 관리 명령을 담당하는 실행 파일 크레이트입니다.

use rust_course_modules_crates::{Task, TaskStatus};

fn main() {
    let task = Task::new(100, "배포 승인", TaskStatus::Pending);

    println!("관리 작업 #{}: {}", task.id(), task.title());
}
