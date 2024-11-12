pub mod db_interactor {
    use sqlite::{State, Statement};
    const DB_PATH: &str = "/Users/miller/coding/projects/apprenticeship-tracker/tracker.db";
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
        ) -> Result<(), ()>;
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
        ) -> Result<(), ()> {
            const USER_ID: i64 = 1;
            let connection = sqlite::open(DB_PATH).unwrap();

            print!("Area {area} | Closes {close_date} | Company {company}\nApplied {date_applied} | Level {level} | Notes {notes} | Pay {pay}\nRequirements {requirements} | Role {role} | Sector {sector}\nStage {stage}\n");

            let mut insert_stmnt: Statement = connection.prepare("INSERT INTO apprenticeship (user_id, area, close_date, company, date_applied, level, notes, pay, requirements, role, sector, stage) VALUES (:user_id, :area, :close_date, :company, :date_applied, :level, :notes, :pay, :requirements, :role, :sector, :stage)").unwrap();

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
            insert_stmnt.next().unwrap();

            Ok(())
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
