// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod db;
use db::db_interactor::{DBInteractor, Interactions};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    const DB: DBInteractor = DBInteractor;
    ui.on_send_entry({
        // clone to enable use in other callbacks
        // let ui_handle = ui_handle.clone();

        // Closures allow inferred input & return Types.
        move |payload| {
            let ui = ui_handle.unwrap();
            println!("{:?}", payload);
            // Destructure payload (user input) tuple to vars
            let (
                area,
                close_date,
                company,
                date_applied,
                level_str,
                notes,
                pay_str,
                requirements,
                role,
                sector,
                stage,
            ) = payload;

            // change level and pay from &str to appropriate T
            let level: i8 = level_str.parse().unwrap_or(0);
            let pay: f64 = pay_str.parse().unwrap_or(0.0);

            match DB.insert_appr(
                area.as_str(),
                close_date.as_str(),
                company.as_str(),
                date_applied.as_str(),
                level,
                notes.as_str(),
                pay,
                requirements.as_str(),
                role.as_str(),
                sector.as_str(),
                stage.as_str(),
            ) {
                Ok(()) => {
                    println!("Success");
                    ui.set_output("Apprenticeship recorded successfully !".into())
                }
                Err(e) => {
                    ui.set_output("Error recording apprenticeship details :(".into());
                    println!("Error with insert: {:?}", e)
                }
            }
        }
        // Date should be format: YYYY-MM-DD // is stored in format DD-MM-YYYY
    });

    // SELECT proof of concept
    match DB.select() {
        Ok(data) => {
            let output = format!("{:?}", data);
            // ui.set_output("Select success".into());
            println!("{}", output.clone());
            ui.set_select_result(output.clone().into());
        }
        Err(e) => {
            ui.set_output("Error fetching data".into());
            println!("Error with select: {:?}", e);
        }
    }
    ui.run()
}
