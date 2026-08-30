/// 작업의 진행 상태입니다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Done,
}

/// 외부에서 사용할 수 있는 작업 타입입니다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, title: impl Into<String>, status: TaskStatus) -> Self {
        Self {
            id,
            title: title.into(),
            status,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> TaskStatus {
        self.status
    }
}
