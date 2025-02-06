pub mod models;

use diesel::prelude::*;
use tauri::{path::BaseDirectory, AppHandle, Manager};

pub type ConnectionType = diesel::sqlite::SqliteConnection;

/// Establish a connection to the database
///
/// This function is used to establish a connection to the database.
/// It uses the `app` handle to get the path to the database file.
/// The database file is located in the `AppLocalData` directory.
/// The database file is named `panel.db`.
pub fn establish_connection(app: AppHandle) -> ConnectionType {
    let database_url = app
        .path()
        .resolve("panel.db", BaseDirectory::AppLocalData)
        .unwrap();

    ConnectionType::establish(&database_url.to_string_lossy())
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url.to_string_lossy()))
}
