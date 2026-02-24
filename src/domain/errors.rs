use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    EmailAlreadyExists,
    InvalidCredentials,
    DatabaseError(String),
    NotFound,
    Unauthorized,
    InvalidInput(String),
    InsufficientBalance,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::EmailAlreadyExists => write!(f, "Email already exists"),
            DomainError::InvalidCredentials => write!(f, "Invalid credentials"),
            DomainError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            DomainError::NotFound => write!(f, "Resource not found"),
            DomainError::Unauthorized => write!(f, "Unauthorized"),
            DomainError::InvalidInput(msg) => write!(f, "{}", msg),
            DomainError::InsufficientBalance => write!(f, "Insufficient balance"),
        }
    }
}

impl std::error::Error for DomainError {}
