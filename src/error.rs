use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Task not found: id={0}")]
    TaskNotFound(u64),

    #[error("Invalid iinput: {0}")]
    InvalidInput(String),
}

pub type AppResult<T> = Result<T, AppError>;
