pub mod service {
    // ANCHOR: child_access
    fn private_key() -> &'static str {
        "rust-2026"
    }

    pub mod api {
        pub fn key_length() -> usize {
            // 자식 모듈에서는 부모 모듈의 비공개 항목을 사용할 수 있습니다.
            super::private_key().len()
        }
    }
    // ANCHOR_END: child_access

    // ANCHOR: private_path
    mod internal {
        pub fn trace_id() -> u64 {
            42
        }
    }

    pub fn trace_id() -> u64 {
        internal::trace_id()
    }
    // ANCHOR_END: private_path
}

fn check_module_visibility() {
    assert_eq!(service::api::key_length(), 9);
    assert_eq!(service::trace_id(), 42);
    // service::internal::trace_id(); // 상위 모듈이 비공개라서 접근할 수 없습니다.
}

// ANCHOR: restricted_visibility
mod jobs {
    mod parser {
        pub(in crate::jobs) fn field_count(input: &str) -> usize {
            input.split('|').count()
        }
    }

    pub(crate) fn field_count(input: &str) -> usize {
        parser::field_count(input)
    }
}
// ANCHOR_END: restricted_visibility

fn check_restricted_visibility() {
    assert_eq!(jobs::field_count("1|문서 작성|done"), 3);
}

mod account {
    // ANCHOR: account
    pub struct Account {
        name: String,
        balance: u64,
    }

    impl Account {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                balance: 0,
            }
        }

        pub fn deposit(&mut self, amount: u64) {
            self.balance += amount;
        }

        pub fn summary(&self) -> String {
            format!("{}: {}원", self.name, self.balance)
        }
    }
    // ANCHOR_END: account
}

mod task {
    // ANCHOR: enum_visibility
    pub enum TaskState {
        Ready,
        Running { worker: String },
        Failed(String),
    }
    // ANCHOR_END: enum_visibility
}

fn main() {
    check_module_visibility();
    check_restricted_visibility();

    // ANCHOR: use
    let mut account = account::Account::new("서준");
    account.deposit(10_000);
    assert_eq!(account.summary(), "서준: 10000원");
    // account.balance = 1_000_000; // 비공개 필드에는 접근할 수 없습니다.
    // ANCHOR_END: use

    // ANCHOR: enum_construct
    use task::TaskState;

    let states = [
        TaskState::Ready,
        TaskState::Running {
            worker: String::from("민서"),
        },
        TaskState::Failed(String::from("입력 오류")),
    ];
    // ANCHOR_END: enum_construct

    // ANCHOR: enum_match
    let labels: Vec<String> = states
        .into_iter()
        .map(|state| match state {
            TaskState::Ready => String::from("대기"),
            TaskState::Running { worker } => format!("실행: {worker}"),
            TaskState::Failed(message) => format!("실패: {message}"),
        })
        .collect();

    assert_eq!(labels, ["대기", "실행: 민서", "실패: 입력 오류"]);
    // ANCHOR_END: enum_match
}
