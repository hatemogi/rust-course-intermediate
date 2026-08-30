//! 패키지의 기본 실행 파일 크레이트입니다.

use rust_course_modules_crates::{parse_task, render_report};

fn main() {
    let tasks = ["1|교재 검수|done", "2|슬라이드 녹화|pending"]
        .into_iter()
        .map(parse_task)
        .collect::<Result<Vec<_>, _>>()
        .expect("예제 작업은 올바른 형식이어야 합니다");

    println!("{}", render_report(&tasks));
}
