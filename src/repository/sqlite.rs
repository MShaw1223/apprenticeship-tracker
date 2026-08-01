use std::path::PathBuf;

use crate::models::Apprenticeship;
use crate::repository::ApprenticeshipRepository;
use sqlite::{Error, State};

pub struct SqliteRepository {
    db_path: PathBuf,
}

impl SqliteRepository {
    pub fn new<P: Into<PathBuf>>(db_path: P) -> Self {
        Self {
            db_path: db_path.into(),
        }
    }
}

impl ApprenticeshipRepository for SqliteRepository {
    fn insert(&self, apprenticeship: &Apprenticeship) -> Result<(), Error> {
        const USER_ID: i64 = 1;
        let connection = sqlite::open(&self.db_path)?;
        let query = "INSERT INTO apprenticeship (user_id, area, close_date, company, date_applied, level, notes, pay, requirements, role, stage) VALUES (:user_id, :area, :close_date, :company, :date_applied, :level, :notes, :pay, :requirements, :role, :stage)";
        let mut statement = connection.prepare(query)?;

        statement.bind((":user_id", USER_ID))?;
        statement.bind((":area", apprenticeship.area.as_str()))?;
        statement.bind((":close_date", apprenticeship.close_date.as_str()))?;
        statement.bind((":company", apprenticeship.company.as_str()))?;
        statement.bind((":date_applied", apprenticeship.date_applied.as_str()))?;
        statement.bind((":level", apprenticeship.level as i64))?;
        statement.bind((":notes", apprenticeship.notes.as_str()))?;
        statement.bind((":pay", apprenticeship.pay))?;
        statement.bind((":requirements", apprenticeship.requirements.as_str()))?;
        statement.bind((":role", apprenticeship.role.as_str()))?;
        statement.bind((":stage", apprenticeship.stage.as_str()))?;

        statement.next()?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Apprenticeship>, Error> {
        const USER_ID: i64 = 1;
        let connection = sqlite::open(&self.db_path)?;
        let query = "SELECT id, area, close_date, company, date_applied, level, notes, pay, requirements, role, stage FROM apprenticeship WHERE user_id = (SELECT id FROM user WHERE id = :id)";
        let mut statement = connection.prepare(query)?;
        statement.bind((":id", USER_ID))?;

        let mut listings = Vec::new();

        while let Ok(State::Row) = statement.next() {
            listings.push(Apprenticeship {
                id: statement.read::<i64, _>("id")? as i32,
                area: statement.read::<String, _>("area")?,
                close_date: statement.read::<String, _>("close_date")?,
                company: statement.read::<String, _>("company")?,
                date_applied: statement.read::<String, _>("date_applied")?,
                level: statement.read::<i64, _>("level")? as i8,
                notes: statement.read::<String, _>("notes")?,
                pay: statement.read::<f64, _>("pay")?,
                requirements: statement.read::<String, _>("requirements")?,
                role: statement.read::<String, _>("role")?,
                stage: statement.read::<String, _>("stage")?,
            });
        }

        Ok(listings)
    }

    fn update_field(&self, field: &str, new_value: &str, row_id: i32) -> Result<(), Error> {
        let connection = sqlite::open(&self.db_path)?;
        let query = format!("UPDATE apprenticeship SET {} = :value WHERE id = :row_id", field);
        let mut statement = connection.prepare(&query)?;

        statement.bind((":value", new_value))?;
        statement.bind((":row_id", row_id as i64))?;
        statement.next()?;
        Ok(())
    }

    fn delete(&self, row_id: i32) -> Result<(), Error> {
        let connection = sqlite::open(&self.db_path)?;
        let query = "DELETE FROM apprenticeship WHERE id = :row_id";
        let mut statement = connection.prepare(query)?;

        statement.bind((":row_id", row_id as i64))?;
        statement.next()?;
        Ok(())
    }
}
