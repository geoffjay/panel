use diesel::prelude::*;
use tauri::{AppHandle, Manager, path::BaseDirectory};

use crate::models::{Dashboard, NewDashboard};

/// Establish a connection to the database
/// 
/// This function is used to establish a connection to the database.
/// It uses the `app` handle to get the path to the database file.
/// The database file is located in the `AppLocalData` directory.
/// The database file is named `panel.db`.
pub fn establish_connection(app: AppHandle) -> SqliteConnection {
    let database_url = app
        .path()
        .resolve("panel.db", BaseDirectory::AppLocalData)
        .unwrap();

    SqliteConnection::establish(&database_url.to_string_lossy())
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url.to_string_lossy()))
}

pub fn get_dashboard(connection: &mut SqliteConnection, dashboard_id: i32) -> Dashboard {
    use crate::schema::dashboards::dsl::*;

    dashboards
        .find(dashboard_id)
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

pub fn delete_dashboard(connection: &mut SqliteConnection, dashboard_id: i32) {
    use crate::schema::dashboards::dsl::*;

    diesel::delete(dashboards.find(dashboard_id)).execute(connection).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel_migrations::MigrationHarness;

    use crate::MIGRATIONS;

    struct TestContext {
        connection: SqliteConnection,
    }

    fn setup_test_environment() -> TestContext {
        let database_url = ":memory:".to_string();
        let mut connection = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

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

    #[test]
    fn test_get_dashboards() {
        let mut context = setup_test_environment();
        let dashboards = get_dashboards(&mut context.connection);

        assert_eq!(dashboards.len(), 0);

        create_dashboard(
            &mut context.connection,
            NewDashboard {
                title: "title 1".to_string(),
                description: "description 1".to_string()
            }
        );

        create_dashboard(
            &mut context.connection,
            NewDashboard {
                title: "title 2".to_string(),
                description: "description 2".to_string()
            }
        );

        let dashboards = get_dashboards(&mut context.connection);
        assert_eq!(dashboards.len(), 2);
    }

    #[test]
    fn test_update_dashboard() {
        let mut context = setup_test_environment();
        let dashboard = create_dashboard(
            &mut context.connection,
            NewDashboard {
                title: "title".to_string(),
                description: "description".to_string()
            }
        );

        let updated_dashboard = update_dashboard(
            &mut context.connection,
            Dashboard {
                id: dashboard.id,
                title: "title 2".to_string(),
                description: "description 2".to_string()
            }
        );

        assert_eq!(updated_dashboard.id, dashboard.id);
        assert_eq!(updated_dashboard.title, "title 2");
        assert_eq!(updated_dashboard.description, "description 2");
    }

    #[test]
    fn test_delete_dashboard() {
        let mut context = setup_test_environment();
        let dashboard = create_dashboard(
            &mut context.connection,
            NewDashboard {
                title: "title".to_string(),
                description: "description".to_string()
            }
        );

        delete_dashboard(&mut context.connection, dashboard.id);
        let dashboards = get_dashboards(&mut context.connection);
        assert_eq!(dashboards.len(), 0);
    }
}
