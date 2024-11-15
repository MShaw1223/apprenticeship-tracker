pub mod db_interactor {
    use sqlite::{Error, State};
    const DB_PATH: &str = "/Users/miller/Coding/projects/apprenticeship-tracker/tracker.db";
    pub struct DBInteractor;
    pub trait Interactions {
        fn insert_appr(
            &self,
            area: &str,
            close_date: &str,
            company: &str,
            date_applied: &str,
            level: i8,
            notes: &str,
            pay: f64,
            requirements: &str,
            role: &str,
            sector: &str,
            stage: &str,
        ) -> Result<(), Error>;
        fn select(&self) -> Result<Vec<Vec<String>>, Error>;
        // change string, () to (), string ?
        fn update(&self, field: &str, new_value: &str, row_id: &i32) -> Result<String, Error>;
        //  fn delete(&self) -> Result<(),()>;
    }
    impl Interactions for DBInteractor {
        fn insert_appr(
            &self,
            area: &str,
            close_date: &str,
            company: &str,
            date_applied: &str,
            level: i8,
            notes: &str,
            pay: f64,
            requirements: &str,
            role: &str,
            sector: &str,
            stage: &str,
        ) -> Result<(), Error> {
            const USER_ID: i64 = 1;
            let connection = match sqlite::open(DB_PATH) {
                Ok(conn) => conn,
                Err(e) => return Err(e),
            };

            print!("Area {area} | Closes {close_date} | Company {company}\nApplied {date_applied} | Level {level} | Notes {notes} | Pay {pay}\nRequirements {requirements} | Role {role} | Sector {sector}\nStage {stage}\n");

            let query = "INSERT INTO apprenticeship (user_id, area, close_date, company, date_applied, level, notes, pay, requirements, role, sector, stage) VALUES (:user_id, :area, :close_date, :company, :date_applied, :level, :notes, :pay, :requirements, :role, :sector, :stage)";
            let mut insert_stmnt = match connection.prepare(query) {
                Ok(stmnt) => stmnt,
                Err(e) => return Err(e),
            };

            insert_stmnt.bind((":user_id", USER_ID)).unwrap();
            insert_stmnt.bind((":area", area)).unwrap();
            insert_stmnt.bind((":close_date", close_date)).unwrap();
            insert_stmnt.bind((":company", company)).unwrap();
            insert_stmnt.bind((":date_applied", date_applied)).unwrap();
            insert_stmnt.bind((":level", level as i64)).unwrap();
            insert_stmnt.bind((":notes", notes)).unwrap();
            insert_stmnt.bind((":pay", pay)).unwrap();
            insert_stmnt.bind((":requirements", requirements)).unwrap();
            insert_stmnt.bind((":role", role)).unwrap();
            insert_stmnt.bind((":sector", sector)).unwrap();
            insert_stmnt.bind((":stage", stage)).unwrap();

            // executes
            match insert_stmnt.next() {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        fn select(&self) -> Result<Vec<Vec<String>>, Error> {
            const USER_ID: i64 = 1;
            let connection = sqlite::open(DB_PATH).unwrap();

            let query =
                "SELECT * FROM apprenticeship WHERE user_id = (SELECT id FROM user where id = :id)";

            let mut statement = connection.prepare(query).unwrap();

            statement.bind((":id", USER_ID)).unwrap();

            let mut payload = Vec::new();

            // IMPORTANT: this works for apprenticeship table only. If any new added in future logic will need adjusting.
            while let Ok(State::Row) = statement.next() {
                let mut row = Vec::new();
                row.push(statement.read::<String, _>("id").unwrap());

                row.push(statement.read::<String, _>("user_id").unwrap());

                row.push(statement.read::<String, _>("company").unwrap());

                row.push(statement.read::<String, _>("role").unwrap());

                row.push(statement.read::<String, _>("pay").unwrap());

                row.push(statement.read::<String, _>("area").unwrap());

                row.push(statement.read::<String, _>("sector").unwrap());

                row.push(statement.read::<String, _>("level").unwrap());

                row.push(statement.read::<String, _>("requirements").unwrap());

                row.push(statement.read::<String, _>("date_applied").unwrap());

                row.push(statement.read::<String, _>("stage").unwrap());

                row.push(statement.read::<String, _>("close_date").unwrap());

                row.push(statement.read::<String, _>("notes").unwrap());

                payload.push(row);
            }

            Ok(payload)
        }
        fn update(&self, field: &str, new_value: &str, row_id: &i32) -> Result<String, Error> {
            println!("F: {:?}, NV: {:?}, RID: {:?}", field, new_value, row_id);
            let connection = match sqlite::open(DB_PATH) {
                Ok(conn) => conn,
                Err(e) => return Err(e),
            };

            // update *table SET *col = *val WHERE *col = *old AND userid = ID
            let query = format!(
                "UPDATE apprenticeship SET {:?} = {:?} WHERE id = :row_id",
                field, new_value
            );

            let mut update_stmnt = match connection.prepare(&query) {
                Ok(stmnt) => stmnt,
                Err(e) => return Err(e),
            };

            // row_id dereffed to be converted to i64 to allow binding
            let new_id: i64 = *row_id as i64;

            update_stmnt.bind((":row_id", new_id)).unwrap();

            let row = match update_stmnt.next() {
                Ok(sqlite::State::Row) => {
                    // If a row is available, read the "Notes" field
                    let notes = update_stmnt.read::<String, _>("notes")?;
                    Ok(notes)
                }
                Ok(sqlite::State::Done) => {
                    // No row found with the given ID? OR finished with iter thru rows
                    Ok("Information updated !".to_string())
                }
                Err(e) => Err(e),
            };
            row
        }
    }
}
