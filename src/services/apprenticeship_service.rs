use crate::models::Apprenticeship;
use crate::repository::ApprenticeshipRepository;
use sqlite::Error;

#[derive(Debug)]
pub enum AppError {
    DbError(Error),
    InvalidInput(String),
    UnsupportedField(String),
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        AppError::DbError(error)
    }
}

pub struct ApprenticeshipService<R: ApprenticeshipRepository> {
    repository: R,
}

impl<R: ApprenticeshipRepository> ApprenticeshipService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create(
        &self,
        area: &str,
        close_date: &str,
        company: &str,
        date_applied: &str,
        level: &str,
        notes: &str,
        pay: &str,
        requirements: &str,
        role: &str,
        stage: &str,
    ) -> Result<(), AppError> {
        let level = level.parse::<i8>().map_err(|_| {
            AppError::InvalidInput(format!("Could not parse level: {}", level))
        })?;

        let pay = pay.parse::<f64>().map_err(|_| {
            AppError::InvalidInput(format!("Could not parse pay: {}", pay))
        })?;

        let apprenticeship = Apprenticeship {
            id: 0,
            area: area.to_string(),
            close_date: close_date.to_string(),
            company: company.to_string(),
            date_applied: date_applied.to_string(),
            level,
            notes: notes.to_string(),
            pay,
            requirements: requirements.to_string(),
            role: role.to_string(),
            stage: stage.to_string(),
        };

        self.repository.insert(&apprenticeship).map_err(AppError::from)
    }

    pub fn list(&self) -> Result<Vec<Apprenticeship>, AppError> {
        self.repository.list().map_err(AppError::from)
    }

    pub fn update_field(
        &self,
        field: &str,
        new_value: &str,
        row_id: &str,
    ) -> Result<(), AppError> {
        let normalized_field = match field {
            "area" | "close_date" | "company" | "date_applied" | "level" | "notes"
            | "pay" | "requirements" | "role" | "stage" => field,
            _ => return Err(AppError::UnsupportedField(field.to_string())),
        };

        let row_id = row_id.parse::<i32>().map_err(|_| {
            AppError::InvalidInput(format!("Could not parse row id: {}", row_id))
        })?;

        self.repository
            .update_field(normalized_field, new_value, row_id)
            .map_err(AppError::from)
    }

    pub fn delete(&self, row_id: &str) -> Result<(), AppError> {
        let row_id = row_id.parse::<i32>().map_err(|_| {
            AppError::InvalidInput(format!("Could not parse row id: {}", row_id))
        })?;

        self.repository.delete(row_id).map_err(AppError::from)
    }
}
