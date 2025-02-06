#![recursion_limit = "256"]

mod app;
mod db;
mod repositories;
mod server;
mod utils;
#[rustfmt::skip]
mod schema;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::thread;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                let mut connection = db::establish_connection(app.handle().clone());
                connection.run_pending_migrations(MIGRATIONS).unwrap();

                let app_handle = app.handle().clone();

                thread::spawn(|| {
                    server::init(app_handle, connection).unwrap();
                });
            }

            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            app::commands::initialize,
            app::commands::fetch_projects,
            app::commands::create_project,
            app::commands::delete_project,
            app::commands::get_dashboard_by_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
