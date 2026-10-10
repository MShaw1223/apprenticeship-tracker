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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_describes_each_error() {
        assert_eq!(
            AppError::InvalidInput("Could not parse pay: lots".to_string()).to_string(),
            "Could not parse pay: lots"
        );
        assert_eq!(
            AppError::UnsupportedField("id".to_string()).to_string(),
            "Cannot update field: id"
        );
        assert_eq!(AppError::NotFound(7).to_string(), "No listing with ID 7");
    }

    #[test]
    fn sqlite_errors_convert_to_db_errors() {
        let error = sqlite::Error {
            code: Some(1),
            message: Some("no such table".to_string()),
        };

        let app_error = AppError::from(error);

        assert!(matches!(app_error, AppError::DbError(_)));
        assert!(app_error.to_string().starts_with("Database error: "));
        assert!(app_error.to_string().contains("no such table"));
    }
}
