pub mod sqlite;

use crate::models::Apprenticeship;
use ::sqlite::Error;

pub trait ApprenticeshipRepository {
    fn insert(&self, apprenticeship: &Apprenticeship) -> Result<(), Error>;
    fn list(&self) -> Result<Vec<Apprenticeship>, Error>;
    fn update_field(&self, field: &str, new_value: &str, row_id: i32) -> Result<(), Error>;
    fn delete(&self, row_id: i32) -> Result<(), Error>;
}
