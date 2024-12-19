use diesel::SqliteConnection;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
pub struct AppState {
    pub tauri: Option<AppHandle>,
    pub db: Option<Arc<Mutex<SqliteConnection>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tauri: None,
            db: None,
        }
    }
}
