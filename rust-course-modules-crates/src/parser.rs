use std::{error::Error, fmt};

use crate::model::{Task, TaskStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseTaskError {
    InvalidFormat,
    InvalidId,
    EmptyTitle,
    InvalidStatus,
}

impl fmt::Display for ParseTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidFormat => "ID|제목|상태 형식이 아닙니다",
            Self::InvalidId => "작업 ID가 올바른 숫자가 아닙니다",
            Self::EmptyTitle => "작업 제목이 비어 있습니다",
            Self::InvalidStatus => "작업 상태는 pending 또는 done이어야 합니다",
        };
        f.write_str(message)
    }
}

impl Error for ParseTaskError {}

pub fn parse_task(input: &str) -> Result<Task, ParseTaskError> {
    let mut fields = input.split('|');
    let id = fields.next().ok_or(ParseTaskError::InvalidFormat)?;
    let title = fields.next().ok_or(ParseTaskError::InvalidFormat)?.trim();
    let status = fields.next().ok_or(ParseTaskError::InvalidFormat)?.trim();
    if fields.next().is_some() {
        return Err(ParseTaskError::InvalidFormat);
    }

    let id = id.trim().parse().map_err(|_| ParseTaskError::InvalidId)?;
    if title.is_empty() {
        return Err(ParseTaskError::EmptyTitle);
    }
    let status = match status {
        "pending" => TaskStatus::Pending,
        "done" => TaskStatus::Done,
        _ => return Err(ParseTaskError::InvalidStatus),
    };
    Ok(Task::new(id, title, status))
}
