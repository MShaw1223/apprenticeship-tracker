use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DbError(sqlite::Error),
    InvalidInput(String),
    UnsupportedField(String),
    NotFound(i64),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DbError(error) => write!(f, "Database error: {}", error),
            AppError::InvalidInput(message) => write!(f, "{}", message),
            AppError::UnsupportedField(field) => write!(f, "Cannot update field: {}", field),
            AppError::NotFound(id) => write!(f, "No listing with ID {}", id),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlite::Error> for AppError {
    fn from(error: sqlite::Error) -> Self {
        AppError::DbError(error)
    }
}
