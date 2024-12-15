use diesel::prelude::*;
use tauri::{AppHandle, Manager, path::BaseDirectory};

use crate::models::{Dashboard, NewDashboard};

pub fn establish_connection(app: AppHandle) -> SqliteConnection {
    let database_url = app
        .path()
        .resolve("panel.db", BaseDirectory::AppLocalData)
        .unwrap();

    SqliteConnection::establish(&database_url.to_string_lossy())
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url.to_string_lossy()))
}

pub fn establish_test_connection() -> SqliteConnection {
    let database_url = "sqlite://test.db".to_string();

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_dashboard(connection: &mut SqliteConnection, id: i32) -> Dashboard {
    use crate::schema::dashboards::dsl::*;

    dashboards
        .find(id)
        .first(connection)
        .unwrap()
}

pub fn get_dashboards(connection: &mut SqliteConnection) -> Vec<Dashboard> {
    use crate::schema::dashboards::dsl::*;

    dashboards.load::<Dashboard>(connection).unwrap()
}

pub fn create_dashboard(connection: &mut SqliteConnection, dashboard: NewDashboard) -> Dashboard {
    use crate::schema::dashboards::dsl::*;

    diesel::insert_into(dashboards)
        .values(&dashboard)
        .returning(Dashboard::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

pub fn update_dashboard(connection: &mut SqliteConnection, dashboard: Dashboard) -> Dashboard {
    use crate::schema::dashboards::dsl::*;

    diesel::update(dashboards.find(dashboard.id))
        .set((
            title.eq(dashboard.title),
            description.eq(dashboard.description)
        ))
        .returning(Dashboard::as_returning())
        .get_result(connection)
        .expect("Error updating dashboard")
}

pub fn delete_dashboard(connection: &mut SqliteConnection, id: i32) {
    use crate::schema::dashboards::dsl::*;

    diesel::delete(dashboards.find(id)).execute(connection).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel_migrations::MigrationHarness;

    use crate::db::establish_test_connection;
    use crate::MIGRATIONS;

    struct TestContext {
        connection: SqliteConnection,
    }

    fn setup_test_environment() -> TestContext {
        let mut connection: SqliteConnection = establish_test_connection();
        connection.run_pending_migrations(MIGRATIONS).unwrap();

        TestContext { connection }
    }

    #[test]
    fn test_get_dashboard() {
        let mut context = setup_test_environment();
        let dashboard = create_dashboard(
            &mut context.connection,
            NewDashboard {
                title: "title".to_string(),
                description: "description".to_string()
            }
        );
        let found_dashboard = get_dashboard(&mut context.connection, dashboard.id);

        assert_eq!(dashboard.id, found_dashboard.id);
        assert_eq!(dashboard.title, found_dashboard.title);
        assert_eq!(dashboard.description, found_dashboard.description);
    }
}
