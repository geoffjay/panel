use std::sync::{Arc, Mutex};
use tauri::AppHandle;

use crate::db::{establish_connection, ConnectionType};

pub mod commands;
pub mod event;

pub struct AppState {
    pub connection: Arc<Mutex<ConnectionType>>,
}

impl AppState {
    pub fn new(app: AppHandle) -> Self {
        let connection = Arc::new(Mutex::new(establish_connection(app)));
        Self { connection }
    }
}
