# Apprenticeship Tracker

A desktop app for keeping track of apprenticeship applications, built with Rust,
[Slint](https://slint.rs/) for the user interface and SQLite for storage.

Listings can be added, viewed, updated and deleted. Each one records the company, role,
pay, area, level, required grades, date applied, stage, close date and notes.

## Usage

Install Rust by following its [getting-started guide](https://www.rust-lang.org/learn/get-started), then:

```
cargo run
```

Listings are stored in `tracker.db` in the directory the app is run from. The file and
its tables are created on first run.

```
cargo test
```

## Layout

```
src/
  main.rs       connects the Slint callbacks to the services
  lib.rs
  models/       data types: Apprenticeship, ApprenticeshipInput, Field, AppError
  services/
    apprenticeship.rs   validating and recording listings
    database.rs         SQLite storage
ui/
  appwindow.slint       the main window
  components/           sidebar, forms, table and dialog
```
