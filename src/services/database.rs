use std::path::Path;

use crate::models::{Apprenticeship, Field};
use sqlite::{Connection, Error, State};

// Users are not a feature yet, so every listing belongs to the same one.
const USER_ID: i64 = 1;

const SCHEMA: &str = "
    CREATE TABLE IF NOT EXISTS user (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR(30) NOT NULL
    );
    CREATE TABLE IF NOT EXISTS apprenticeship (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INT NOT NULL,
        area VARCHAR(100) NOT NULL,
        close_date TEXT NOT NULL,
        company VARCHAR(100) NOT NULL,
        date_applied TEXT NOT NULL,
        level SMALLINT NOT NULL,
        notes TEXT NOT NULL,
        pay REAL NOT NULL,
        requirements TEXT,
        role VARCHAR(100) NOT NULL,
        stage VARCHAR(50) NOT NULL,
        FOREIGN KEY (user_id) REFERENCES user(id)
    );
";

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let connection = sqlite::open(path)?;
        connection.execute(SCHEMA)?;
        Ok(Database { connection })
    }

    pub fn insert(&self, apprenticeship: &Apprenticeship) -> Result<(), Error> {
        let query = "INSERT INTO apprenticeship (user_id, area, close_date, company, date_applied, level, notes, pay, requirements, role, stage) VALUES (:user_id, :area, :close_date, :company, :date_applied, :level, :notes, :pay, :requirements, :role, :stage)";
        let mut statement = self.connection.prepare(query)?;

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

    pub fn list(&self) -> Result<Vec<Apprenticeship>, Error> {
        let query = "SELECT id, area, close_date, company, date_applied, level, notes, pay, requirements, role, stage FROM apprenticeship WHERE user_id = :user_id";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((":user_id", USER_ID))?;

        let mut listings = Vec::new();

        while let State::Row = statement.next()? {
            listings.push(Apprenticeship {
                id: statement.read::<i64, _>("id")?,
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

    // Returns the number of rows changed, which is 0 when the id does not exist.
    pub fn update_field(&self, field: Field, new_value: &str, row_id: i64) -> Result<usize, Error> {
        let query = format!(
            "UPDATE apprenticeship SET {} = :value WHERE id = :row_id",
            field.column()
        );
        let mut statement = self.connection.prepare(&query)?;

        statement.bind((":value", new_value))?;
        statement.bind((":row_id", row_id))?;
        statement.next()?;
        Ok(self.connection.change_count())
    }

    // Returns the number of rows deleted, which is 0 when the id does not exist.
    pub fn delete(&self, row_id: i64) -> Result<usize, Error> {
        let query = "DELETE FROM apprenticeship WHERE id = :row_id";
        let mut statement = self.connection.prepare(query)?;

        statement.bind((":row_id", row_id))?;
        statement.next()?;
        Ok(self.connection.change_count())
    }
}
