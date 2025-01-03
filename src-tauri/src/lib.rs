#![recursion_limit = "256"]

mod db;
mod server;
mod utils;
#[rustfmt::skip]
mod schema;

use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde::Serialize;
use std::fmt;
use std::thread;
use tauri::{ipc::Channel, AppHandle};

use crate::db::establish_connection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
#[allow(clippy::enum_variant_names)]
enum AppEvent<'a> {
    #[serde(rename_all = "camelCase")]
    LoadingStarted { id: usize },
    #[serde(rename_all = "camelCase")]
    LoadingProgress {
        id: usize,
        percent: usize,
        message: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    LoadingFinished { id: usize },
}

impl fmt::Display for AppEvent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEvent::LoadingStarted { id } => write!(f, "LoadingStarted(id: {})", id),
            AppEvent::LoadingProgress {
                id,
                percent,
                message,
            } => write!(f, "LoadingProgress(id: {}, {}%: {})", id, percent, message),
            AppEvent::LoadingFinished { id } => write!(f, "LoadingFinished(id: {})", id),
        }
    }
}

#[tauri::command]
fn initialize(_app: AppHandle, on_event: Channel<AppEvent>) {
    let id = 1;

    on_event.send(AppEvent::LoadingStarted { id }).unwrap();

    for percent in [10, 30, 75, 90, 100] {
        on_event
            .send(AppEvent::LoadingProgress {
                id,
                percent,
                message: "Loading...",
            })
            .unwrap();
    }

    on_event.send(AppEvent::LoadingFinished { id }).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                let mut connection: SqliteConnection = establish_connection(app.handle().clone());
                connection.run_pending_migrations(MIGRATIONS).unwrap();

                let app_handle = app.handle().clone();

                thread::spawn(|| {
                    server::init(app_handle, connection).unwrap();
                });
            }

            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![initialize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
