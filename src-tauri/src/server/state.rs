use diesel::SqliteConnection;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

#[derive(Default)]
pub struct AppState {
    #[allow(dead_code)]
    pub tauri: Option<AppHandle>,
    pub db: Option<Arc<Mutex<SqliteConnection>>>,
}
