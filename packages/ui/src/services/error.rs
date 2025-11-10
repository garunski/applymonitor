//! Service error types

use std::fmt;

/// Service error types for API calls
#[derive(Debug, Clone)]
pub enum ServiceError {
    /// Network/connection errors
    Network(String),
    /// JSON parsing errors
    Parse(String),
    /// Server errors with status code
    Server(u16, String),
    /// 404 Not Found
    NotFound,
    /// 401 Unauthorized
    Unauthorized,
    /// Other unknown errors
    Unknown(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::Network(msg) => write!(f, "Network error: {}", msg),
            ServiceError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ServiceError::Server(code, msg) => write!(f, "Server error ({}): {}", code, msg),
            ServiceError::NotFound => write!(f, "Resource not found"),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
            ServiceError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}
