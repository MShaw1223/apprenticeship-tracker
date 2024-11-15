// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod db;

use std::collections::HashMap;

use db::db_interactor::{DBInteractor, Interactions};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    const DB: DBInteractor = DBInteractor;

    ui.on_send_entry({
        // clone to enable use in other callbacks
        let ui_handle = ui_handle.clone();

        move |payload| {
            let ui = ui_handle.unwrap();
            println!("Data recieved in main.rs closure: {:?}", payload);
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
                sector.as_str(),
                stage.as_str(),
            ) {
                Ok(()) => {
                    ui.set_output("Apprenticeship recorded successfully !".into());
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
        let ui_handle = ui_handle.clone();
        move || {
            let ui = ui_handle.unwrap();
            let mut cols: HashMap<i8, &str> = HashMap::new();
            cols.insert(0, "Listing_ID");
            cols.insert(1, "User_ID");
            cols.insert(2, "Company");
            cols.insert(3, "Role");
            cols.insert(4, "Pay");
            cols.insert(5, "Area");
            cols.insert(6, "Sector");
            cols.insert(7, "Level");
            cols.insert(8, "Required Grades");
            cols.insert(9, "Date Applied");
            cols.insert(10, "Stage");
            cols.insert(11, "Close Date");
            cols.insert(12, "Notes");
            match DB.select() {
                Ok(listings) => {
                    let mut formatted_vec: Vec<Vec<String>> = Vec::new();

                    for listing in listings {
                        let mut temp: Vec<String> = Vec::new();
                        for (index, col_value) in listing.iter().enumerate() {
                            if let Ok(index) = i8::try_from(index) {
                                if let Some(col) = cols.get(&index) {
                                    temp.push(format!("{} : {}", col, col_value));
                                }
                            }
                        }
                        formatted_vec.push(temp);
                    }
                    ui.set_select_result(format!("{:?}", formatted_vec).into());
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
        let ui_handle = ui_handle.clone();
        move |update_payload| {
            let ui = ui_handle.unwrap();
            println!("Payload recieved: {:?}", update_payload);
            let (field, new_value, id) = update_payload;
            match DB.update(&field, &new_value, &id) {
                Ok(msg) => {
                    println!("Message recieved in main.rs: {:?}", msg);
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
    ui.run()
}
