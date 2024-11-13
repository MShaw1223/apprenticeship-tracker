pub mod db_interactor {
    use sqlite::State;
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
        ) -> Result<(), String>;
        fn select(&self) -> Result<Vec<std::string::String>, ()>;
        //  fn update(&self) -> Result<(),()>;
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
        ) -> Result<(), String> {
            const USER_ID: i64 = 1;
            let connection = match sqlite::open(DB_PATH) {
                Ok(conn) => conn,
                Err(e) => return Err(format!("Failed to connect to the database: {}", e)),
            };

            print!("Area {area} | Closes {close_date} | Company {company}\nApplied {date_applied} | Level {level} | Notes {notes} | Pay {pay}\nRequirements {requirements} | Role {role} | Sector {sector}\nStage {stage}\n");

            let query = "INSERT INTO apprenticeship (user_id, area, close_date, company, date_applied, level, notes, pay, requirements, role, sector, stage) VALUES (:user_id, :area, :close_date, :company, :date_applied, :level, :notes, :pay, :requirements, :role, :sector, :stage)";
            let mut insert_stmnt = match connection.prepare(query) {
                Ok(stmnt) => stmnt,
                Err(e) => return Err(format!("Failed to prepare insert statement: {}", e)),
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
                Err(e) => Err(format!("Error executing insert statement: {}", e)),
            }
        }
        fn select(&self) -> Result<Vec<std::string::String>, ()> {
            const USER_ID: i64 = 1;
            let connection = sqlite::open(DB_PATH).unwrap();

            let query =
                "SELECT * FROM apprenticeship WHERE user_id = (SELECT id FROM user where id = :id)";

            let mut statement = connection.prepare(query).unwrap();

            statement.bind((":id", USER_ID)).unwrap();

            let mut payload = Vec::new();

            while let Ok(State::Row) = statement.next() {
                payload.push(statement.read::<String, _>("id").unwrap());

                payload.push(statement.read::<String, _>("user_id").unwrap());

                payload.push(statement.read::<String, _>("company").unwrap());

                payload.push(statement.read::<String, _>("role").unwrap());

                payload.push(statement.read::<String, _>("pay").unwrap());

                payload.push(statement.read::<String, _>("area").unwrap());

                payload.push(statement.read::<String, _>("sector").unwrap());

                payload.push(statement.read::<String, _>("level").unwrap());

                payload.push(statement.read::<String, _>("requirements").unwrap());

                payload.push(statement.read::<String, _>("date_applied").unwrap());

                payload.push(statement.read::<String, _>("stage").unwrap());

                payload.push(statement.read::<String, _>("close_date").unwrap());

                payload.push(statement.read::<String, _>("notes").unwrap());
            }

            Ok(payload)
        }
    }
}
