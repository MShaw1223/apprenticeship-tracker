// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::rc::Rc;

use apprenticeship_tracker::repository::sqlite::SqliteRepository;
use apprenticeship_tracker::services::ApprenticeshipService;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let repository = SqliteRepository::new("tracker.db");
    let service = Rc::new(ApprenticeshipService::new(repository));

    ui.on_insert_job_call({
        let ui_handle = ui.as_weak();
        let service = service.clone();

        move |payload| {
            let ui = ui_handle.unwrap();
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

            let result = service.create(
                area.as_str(),
                close_date.as_str(),
                company.as_str(),
                date_applied.as_str(),
                level_str.as_str(),
                notes.as_str(),
                pay_str.as_str(),
                requirements.as_str(),
                role.as_str(),
                stage.as_str(),
            );

            match result {
                Ok(()) => ui.set_output("Job recorded !".into()),
                Err(e) => ui.set_output(format!("Error recording apprenticeship details :(. Error: {:?}", e).into()),
            }
        }
    });

    ui.on_select_call({
        let ui_handle = ui.as_weak();
        let service = service.clone();

        move || {
            let ui = ui_handle.unwrap();
            match service.list() {
                Ok(listings) => {
                    let rows: slint::ModelRc<slint::ModelRc<slint::StandardListViewItem>> =
                        slint::ModelRc::new(slint::VecModel::from(
                            listings
                                .into_iter()
                                .map(|appr| {
                                    let row = vec![
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.id.to_string()),
                                        ),
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.company),
                                        ),
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.role),
                                        ),
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.pay.to_string()),
                                        ),
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.area),
                                        ),
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.level.to_string()),
                                        ),
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.requirements),
                                        ),
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.date_applied),
                                        ),
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.stage),
                                        ),
                                        slint::StandardListViewItem::from(
                                            slint::SharedString::from(appr.notes),
                                        ),
                                    ];
                                    slint::ModelRc::new(slint::VecModel::from(row))
                                })
                                .collect::<Vec<_>>(),
                        ));
                    ui.set_select_result(rows);
                }
                Err(e) => {
                    ui.set_output(format!("Error fetching listings: {:?}", e).into());
                    ui.set_show(true);
                }
            }
        }
    });

    ui.on_update_call({
        let ui_handle = ui.as_weak();
        let service = service.clone();

        move |update_payload| {
            let ui = ui_handle.unwrap();
            let (field, new_value, id) = update_payload;
            match service.update_field(&field, &new_value, &id) {
                Ok(()) => {
                    ui.set_output("Information updated !".into());
                    ui.set_show(true);
                }
                Err(e) => {
                    ui.set_output(format!("Error updating information: {:?}", e).into());
                    ui.set_show(true);
                }
            }
        }
    });

    ui.on_delete_call({
        let ui_handle = ui.as_weak();
        let service = service;

        move |payload| {
            let ui = ui_handle.unwrap();
            let (id,) = payload;
            match service.delete(&id) {
                Ok(()) => ui.set_output("Deletion Successful.".into()),
                Err(e) => ui.set_output(format!("Error deleting job. Error: {:?}", e).into()),
            }
        }
    });

    ui.run()
}
