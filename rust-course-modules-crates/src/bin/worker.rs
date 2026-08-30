//! 작업 처리를 담당하는 실행 파일 크레이트입니다.

use rust_course_modules_crates::parse_task;

fn main() {
    let task = parse_task("200|보고서 생성|pending").expect("예제 작업은 올바른 형식이어야 합니다");

    println!("작업 #{} 처리: {}", task.id(), task.title());
}
