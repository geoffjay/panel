use std::sync::{Arc, Mutex};
use tauri::AppHandle;

use crate::db::{establish_connection, ConnectionType};

pub mod commands;
pub mod event;

#[allow(dead_code)]
pub struct AppState {
    pub connection: Arc<Mutex<ConnectionType>>,
}

impl AppState {
    #[allow(unused)]
    pub fn new(app: AppHandle) -> Self {
        let connection = Arc::new(Mutex::new(establish_connection(app)));
        Self { connection }
    }
}
