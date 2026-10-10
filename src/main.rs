// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::error::Error;
use std::rc::Rc;

use apprenticeship_tracker::models::{Apprenticeship, ApprenticeshipInput};
use apprenticeship_tracker::services::{apprenticeship, database::Database};
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
slint::include_modules!();

const DB_PATH: &str = "tracker.db";

// Column order must match the columns declared in ui/components/table.slint
fn table_row(appr: Apprenticeship) -> ModelRc<StandardListViewItem> {
    let cells = [
        appr.id.to_string(),
        appr.company,
        appr.role,
        appr.pay.to_string(),
        appr.area,
        appr.level.to_string(),
        appr.requirements,
        appr.date_applied,
        appr.close_date,
        appr.stage,
        appr.notes,
    ]
    .map(|cell| StandardListViewItem::from(SharedString::from(cell)));

    ModelRc::new(VecModel::from(cells.to_vec()))
}

fn show_output(ui: &AppWindow, message: String) {
    ui.set_output(message.into());
    ui.set_show(true);
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let db = Rc::new(Database::open(DB_PATH)?);

    ui.on_insert_job_call({
        let ui_handle = ui.as_weak();
        let db = db.clone();

        move |payload| {
            let ui = ui_handle.unwrap();
            // Slint passes the payload as a tuple ordered alphabetically by field name
            let (
                area,
                close_date,
                company,
                date_applied,
                level,
                notes,
                pay,
                requirements,
                role,
                stage,
            ) = payload;

            let input = ApprenticeshipInput {
                area: area.into(),
                close_date: close_date.into(),
                company: company.into(),
                date_applied: date_applied.into(),
                level: level.into(),
                notes: notes.into(),
                pay: pay.into(),
                requirements: requirements.into(),
                role: role.into(),
                stage: stage.into(),
            };

            let result = apprenticeship::create(&db, input);
            match &result {
                Ok(()) => show_output(&ui, "Job recorded !".to_string()),
                Err(e) => show_output(
                    &ui,
                    format!("Error recording apprenticeship details :(. Error: {}", e),
                ),
            }
            result.is_ok()
        }
    });

    ui.on_select_call({
        let ui_handle = ui.as_weak();
        let db = db.clone();

        move || {
            let ui = ui_handle.unwrap();
            match apprenticeship::list(&db) {
                Ok(listings) => {
                    let rows: Vec<_> = listings.into_iter().map(table_row).collect();
                    ui.set_select_result(ModelRc::new(VecModel::from(rows)));
                }
                Err(e) => show_output(&ui, format!("Error fetching listings: {}", e)),
            }
        }
    });

    ui.on_update_call({
        let ui_handle = ui.as_weak();
        let db = db.clone();

        move |payload| {
            let ui = ui_handle.unwrap();
            let (field, new_value, row_id) = payload;
            let result = apprenticeship::update_field(&db, &field, &new_value, &row_id);
            match &result {
                Ok(()) => show_output(&ui, "Information updated !".to_string()),
                Err(e) => show_output(&ui, format!("Error updating information: {}", e)),
            }
            result.is_ok()
        }
    });

    ui.on_delete_call({
        let ui_handle = ui.as_weak();

        move |payload| {
            let ui = ui_handle.unwrap();
            let (row_id,) = payload;
            let result = apprenticeship::delete(&db, &row_id);
            match &result {
                Ok(()) => show_output(&ui, "Deletion Successful.".to_string()),
                Err(e) => show_output(&ui, format!("Error deleting job. Error: {}", e)),
            }
            result.is_ok()
        }
    });

    // The app opens on the listings page, so load them straight away
    ui.invoke_select_call();

    ui.run()?;
    Ok(())
}
