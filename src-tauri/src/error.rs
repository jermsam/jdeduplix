use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCode {
    SerializationError,
    DeserializationError,
    StrategyUpdateError,
    InvalidInput,
    InternalError,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::SerializationError => write!(f, "E001"),
            ErrorCode::DeserializationError => write!(f, "E002"),
            ErrorCode::StrategyUpdateError => write!(f, "E003"),
            ErrorCode::InvalidInput => write!(f, "E004"),
            ErrorCode::InternalError => write!(f, "E005"),
        }
    }
}

impl AppError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::new(
            ErrorCode::SerializationError,
            format!("JSON serialization error: {}", err)
        )
    }
}

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        serde_json::to_string(&err).unwrap_or_else(|_| 
            format!("{{\"code\":\"{}\",\"message\":\"{}\"}}", 
                ErrorCode::InternalError, 
                "Failed to serialize error"
            )
        )
    }
}
