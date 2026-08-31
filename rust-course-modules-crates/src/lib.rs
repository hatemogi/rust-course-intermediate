//! Rust 패키지·크레이트·모듈 강의의 종합 실습 라이브러리입니다.
//!
//! 이 실습에서는 `model.rs`, `parser.rs`, `report.rs`의 구현을 수정하지 않습니다.
//! 크레이트 루트인 이 파일에서 다음 작업을 수행합니다.
//!
//! 1. 세 파일을 비공개 하위 모듈로 연결합니다.
//! 2. 외부 사용자가 필요한 타입과 함수만 크레이트 루트에 재공개합니다.
//!
//! 외부 사용자는 다음처럼 크레이트 루트의 공개 API를 사용할 수 있어야 합니다.
//!
//! ```
//! use rust_course_modules_crates::{Task, TaskStatus, parse_task, render_report};
//!
//! let task = parse_task("1|문서 작성|done").unwrap();
//! assert_eq!(task.status(), TaskStatus::Done);
//! assert_eq!(render_report(&[task]), "완료 1개 / 대기 0개");
//! ```
//!
//! 반면 구현 모듈 자체는 외부에서 접근할 수 없어야 합니다.
//!
//! ```compile_fail
//! use rust_course_modules_crates::model::Task;
//! # fn main() {}
//! ```
//!
//! ```compile_fail
//! use rust_course_modules_crates::parser::parse_task;
//! # fn main() {}
//! ```
//!
//! ```compile_fail
//! use rust_course_modules_crates::report::render_report;
//! # fn main() {}
//! ```

// TODO: 세 구현 파일을 비공개 하위 모듈로 연결하세요.

// TODO: 외부 사용자가 필요한 타입과 함수만 크레이트 루트에 재공개하세요.
