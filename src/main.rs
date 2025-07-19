// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod db;

use db::db_interactor::{DBInteractor, Interactions};
use slint::ComponentHandle;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    const DB: DBInteractor = DBInteractor;

    ui.on_insert_job_call({
        let ui_handle = ui.as_weak();

        move |payload| {
            let ui = ui_handle.unwrap();
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
                stage,
            ) = payload;

            let level: i8 = match level_str.parse() {
                Ok(parsed) => parsed,
                Err(e) => {
                    println!("Error parsing level: {:?}", e);
                    // 0 as default
                    0
                }
            };
            let pay: f64 = match pay_str.parse() {
                Ok(parsed) => parsed,
                Err(e) => {
                    println!("Error parsing pay: {:?}", e);
                    0.0
                }
            };

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
                stage.as_str(),
            ) {
                Ok(msg) => {
                    ui.set_output(msg.into());
                }
                Err(e) => {
                    let ret_str =
                        format!("Error recording apprenticeship details :(. Error: {:?}", e);
                    ui.set_output(ret_str.into());
                }
            }
        }
        // Date should be format: YYYY-MM-DD // is stored in format DD-MM-YYYY
    });
    ui.on_select_call({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            match DB.select() {
                Ok(listings) => {
                    ui.set_select_result(listings);
                }
                Err(e) => {
                    let ret_str = format!("Error fetching listings: {:?}", e);
                    ui.set_output(ret_str.into());
                    ui.set_show(true);
                }
            }
        }
    });
    ui.on_update_call({
        let ui_handle = ui.as_weak();
        move |update_payload| {
            let ui = ui_handle.unwrap();
            let (field, new_value, id) = update_payload;
            let converted_id = id.parse::<i32>().unwrap();
            match DB.update(&field, &new_value, &converted_id) {
                Ok(msg) => {
                    ui.set_output(msg.into());
                    ui.set_show(true);
                }
                Err(e) => {
                    let ret_str = format!("Error updating information: {:?}", e);
                    ui.set_output(ret_str.into());
                    ui.set_show(true);
                }
            }
        }
    });
    ui.on_delete_call({
        let ui_handle = ui.as_weak();
        move |payload| {
            let ui = ui_handle.unwrap();
            let (id,) = payload;
            let row_id: i32 = match id.parse() {
                Ok(parsed) => parsed,
                Err(e) => {
                    println!("Error parsing the row id: {:?}", e);
                    0
                }
            };
            match DB.delete(&row_id) {
                Ok(msg) => {
                    ui.set_output(msg.into());
                }
                Err(e) => {
                    let ret_str = format!("Error deleting job. Error: {:?}", e);
                    ui.set_output(ret_str.into());
                }
            }
        }
    });

    ui.run()
}
