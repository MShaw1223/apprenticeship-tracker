use crate::models::{AppError, Apprenticeship, ApprenticeshipInput, Field};
use crate::services::database::Database;

pub fn list(db: &Database) -> Result<Vec<Apprenticeship>, AppError> {
    Ok(db.list()?)
}

fn parse_level(level: &str) -> Result<i8, AppError> {
    level
        .trim()
        .parse()
        .map_err(|_| AppError::InvalidInput(format!("Could not parse level: {}", level)))
}

fn parse_pay(pay: &str) -> Result<f64, AppError> {
    pay.trim()
        .parse()
        .map_err(|_| AppError::InvalidInput(format!("Could not parse pay: {}", pay)))
}

fn parse_row_id(row_id: &str) -> Result<i64, AppError> {
    row_id
        .trim()
        .parse()
        .map_err(|_| AppError::InvalidInput(format!("Could not parse row id: {}", row_id)))
}

pub fn create(db: &Database, input: ApprenticeshipInput) -> Result<(), AppError> {
    let apprenticeship = Apprenticeship {
        id: 0,
        level: parse_level(&input.level)?,
        pay: parse_pay(&input.pay)?,
        area: input.area,
        close_date: input.close_date,
        company: input.company,
        date_applied: input.date_applied,
        notes: input.notes,
        requirements: input.requirements,
        role: input.role,
        stage: input.stage,
    };

    Ok(db.insert(&apprenticeship)?)
}

pub fn update_field(
    db: &Database,
    field: &str,
    new_value: &str,
    row_id: &str,
) -> Result<(), AppError> {
    let field =
        Field::from_label(field).ok_or_else(|| AppError::UnsupportedField(field.to_string()))?;
    let row_id = parse_row_id(row_id)?;

    // Numeric columns are checked here so a bad value cannot reach the database.
    match field {
        Field::Level => {
            parse_level(new_value)?;
        }
        Field::Pay => {
            parse_pay(new_value)?;
        }
        _ => {}
    }

    match db.update_field(field, new_value.trim(), row_id)? {
        0 => Err(AppError::NotFound(row_id)),
        _ => Ok(()),
    }
}

pub fn delete(db: &Database, row_id: &str) -> Result<(), AppError> {
    let row_id = parse_row_id(row_id)?;

    match db.delete(row_id)? {
        0 => Err(AppError::NotFound(row_id)),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ApprenticeshipInput {
        ApprenticeshipInput {
            area: "London".to_string(),
            close_date: "01/01/2027".to_string(),
            company: "Acme".to_string(),
            date_applied: "10/10/2026".to_string(),
            level: "6".to_string(),
            notes: "None".to_string(),
            pay: "25000".to_string(),
            requirements: "AAB".to_string(),
            role: "Software Engineer".to_string(),
            stage: "Applied".to_string(),
        }
    }

    fn db_with_listing() -> (Database, i64) {
        let db = Database::open(":memory:").unwrap();
        create(&db, input()).unwrap();
        let id = list(&db).unwrap()[0].id;
        (db, id)
    }

    #[test]
    fn create_stores_a_listing() {
        let (db, _) = db_with_listing();
        let listings = list(&db).unwrap();

        assert_eq!(listings.len(), 1);
        assert_eq!(listings[0].company, "Acme");
        assert_eq!(listings[0].level, 6);
        assert_eq!(listings[0].pay, 25000.0);
    }

    #[test]
    fn create_rejects_unparseable_numbers() {
        let db = Database::open(":memory:").unwrap();

        let bad_pay = ApprenticeshipInput {
            pay: "lots".to_string(),
            ..input()
        };
        let bad_level = ApprenticeshipInput {
            level: "Level".to_string(),
            ..input()
        };

        assert!(matches!(
            create(&db, bad_pay),
            Err(AppError::InvalidInput(_))
        ));
        assert!(matches!(
            create(&db, bad_level),
            Err(AppError::InvalidInput(_))
        ));
        assert!(list(&db).unwrap().is_empty());
    }

    #[test]
    fn update_field_accepts_form_labels() {
        let (db, id) = db_with_listing();
        let id = id.to_string();

        update_field(&db, "Company", "Globex", &id).unwrap();
        update_field(&db, "Grades", "ABB", &id).unwrap();
        update_field(&db, "Date Applied", "11/10/2026", &id).unwrap();
        update_field(&db, "Pay", "30000", &id).unwrap();

        let listing = &list(&db).unwrap()[0];
        assert_eq!(listing.company, "Globex");
        assert_eq!(listing.requirements, "ABB");
        assert_eq!(listing.date_applied, "11/10/2026");
        assert_eq!(listing.pay, 30000.0);
    }

    #[test]
    fn update_field_rejects_bad_input() {
        let (db, id) = db_with_listing();
        let id = id.to_string();

        assert!(matches!(
            update_field(&db, "id", "7", &id),
            Err(AppError::UnsupportedField(_))
        ));
        assert!(matches!(
            update_field(&db, "Pay", "lots", &id),
            Err(AppError::InvalidInput(_))
        ));
        assert!(matches!(
            update_field(&db, "Company", "Globex", "abc"),
            Err(AppError::InvalidInput(_))
        ));
        assert!(matches!(
            update_field(&db, "Company", "Globex", "999"),
            Err(AppError::NotFound(999))
        ));
        assert_eq!(list(&db).unwrap()[0].company, "Acme");
    }

    #[test]
    fn delete_removes_a_listing() {
        let (db, id) = db_with_listing();

        delete(&db, &id.to_string()).unwrap();

        assert!(list(&db).unwrap().is_empty());
        assert!(matches!(
            delete(&db, &id.to_string()),
            Err(AppError::NotFound(_))
        ));
    }
}
