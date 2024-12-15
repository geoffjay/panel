use std::sync::Mutex;
use tauri::AppHandle;
use diesel::SqliteConnection;
pub struct AppState {
    pub tauri: Option<AppHandle>,
    pub db: Option<Mutex<SqliteConnection>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tauri: None,
            db: None,
        }
    }
}
