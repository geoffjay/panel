mod server;

use serde::Serialize;
use std::fmt;
use std::thread;
use tauri::{ipc::Channel, AppHandle};
use tauri_plugin_sql::{Migration, MigrationKind};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
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

impl<'a> fmt::Display for AppEvent<'a> {
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
                percent: percent,
                message: "Loading...",
            })
            .unwrap();
    }

    on_event.send(AppEvent::LoadingFinished { id }).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                let app_handle = app.handle().clone();

                thread::spawn(|| {
                    server::init(app_handle).unwrap();
                });
            }

            Ok(())
        })
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:panel.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![initialize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
