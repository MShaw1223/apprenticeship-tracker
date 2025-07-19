slint::include_modules!();

pub mod db_interactor {
    use std::vec;

    use slint::{ModelRc, StandardListViewItem, VecModel};
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
            stage: &str,
        ) -> Result<String, Error>;
        fn select(&self) -> Result<ModelRc<ModelRc<StandardListViewItem>>, Error>;
        fn update(&self, field: &str, new_value: &str, row_id: &i32) -> Result<String, Error>;
        fn delete(&self, row_id: &i32) -> Result<String, Error>;
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
            stage: &str,
        ) -> Result<String, Error> {
            const USER_ID: i64 = 1;
            let connection = match sqlite::open(DB_PATH) {
                Ok(conn) => conn,
                Err(e) => return Err(e),
            };

            let query = "INSERT INTO apprenticeship (user_id, area, close_date, company, date_applied, level, notes, pay, requirements, role, stage) VALUES (:user_id, :area, :close_date, :company, :date_applied, :level, :notes, :pay, :requirements, :role, :stage)";
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
            insert_stmnt.bind((":stage", stage)).unwrap();

            match insert_stmnt.next() {
                Ok(_) => Ok("Job recorded !".to_string()),
                Err(e) => Err(e),
            }
        }
        fn select(&self) -> Result<ModelRc<ModelRc<StandardListViewItem>>, Error> {
            const USER_ID: i64 = 1;
            let connection = sqlite::open(DB_PATH).unwrap();

            let query =
                "SELECT * FROM apprenticeship WHERE user_id = (SELECT id FROM user where id = :id)";

            let mut statement = connection.prepare(query).unwrap();

            statement.bind((":id", USER_ID)).unwrap();

            let mut payload = Vec::new();

            while let Ok(State::Row) = statement.next() {
                payload.push(vec![
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("id").unwrap(),
                    )),
                    // StandardListViewItem::from(slint::SharedString::from(
                    //     statement.read::<String, _>("user_id").unwrap(),
                    // )),
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("company").unwrap(),
                    )),
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("role").unwrap(),
                    )),
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("pay").unwrap(),
                    )),
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("area").unwrap(),
                    )),
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("level").unwrap(),
                    )),
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("requirements").unwrap(),
                    )),
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("date_applied").unwrap(),
                    )),
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("stage").unwrap(),
                    )),
                    // StandardListViewItem::from(slint::SharedString::from(
                    //     statement.read::<String, _>("close_date").unwrap(),
                    // )),
                    StandardListViewItem::from(slint::SharedString::from(
                        statement.read::<String, _>("notes").unwrap(),
                    )),
                ]);
            }
            let row_models: Vec<ModelRc<StandardListViewItem>> = payload
                .into_iter()
                .map(|row| ModelRc::new(VecModel::from(row)))
                .collect();
            let vec_model = VecModel::from(row_models);
            let model_rc: ModelRc<ModelRc<StandardListViewItem>> = ModelRc::new(vec_model);
            Ok(model_rc)
        }
        fn update(&self, field: &str, new_value: &str, row_id: &i32) -> Result<String, Error> {
            let connection = match sqlite::open(DB_PATH) {
                Ok(conn) => conn,
                Err(e) => return Err(e),
            };

            // update *table SET *col = *val WHERE *col = *old [AND userid = ID]**Assumed as users are not a feature
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
        fn delete(&self, row_id: &i32) -> Result<String, Error> {
            let connection = match sqlite::open(DB_PATH) {
                Ok(conn) => conn,
                Err(e) => return Err(e),
            };
            let query = "DELETE FROM apprenticeship WHERE id = :row_id";

            let mut delete_stmnt = match connection.prepare(&query) {
                Ok(stmnt) => stmnt,
                Err(e) => return Err(e),
            };

            // row_id dereffed to be converted to i64 to allow binding
            let new_id: i64 = *row_id as i64;

            delete_stmnt.bind((":row_id", new_id)).unwrap();

            let row = match delete_stmnt.next() {
                Ok(sqlite::State::Row) => {
                    let notes = delete_stmnt.read::<String, _>("notes")?;
                    Ok(notes)
                }
                Ok(sqlite::State::Done) => Ok("Deletion Successful.".to_string()),
                Err(e) => Err(e),
            };
            row
        }
    }
}
