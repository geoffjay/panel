use diesel::prelude::*;

use crate::db::models::{CreateDashboard, Dashboard};

/// Get a dashboard by its id
///
/// This function is used to get a dashboard by its id.
/// It uses the `connection` to get the dashboard from the database.
/// The `id` is the id of the dashboard to get.
#[allow(dead_code)]
pub fn find_dashboard(
    connection: &mut SqliteConnection,
    id: i32,
) -> Result<Dashboard, diesel::result::Error> {
    use crate::schema::dashboards::dsl;

    dsl::dashboards.find(id).first(connection)
}

/// Get all dashboards
///
/// This function is used to get all dashboards.
/// It uses the `connection` to get the dashboards from the database.
pub fn find_dashboards(
    connection: &mut SqliteConnection,
) -> Result<Vec<Dashboard>, diesel::result::Error> {
    use crate::schema::dashboards::dsl;

    dsl::dashboards.load::<Dashboard>(connection)
}

/// Create a dashboard
///
/// This function is used to create a dashboard.
/// It uses the `connection` to create the dashboard in the database.
/// The `dashboard` is the dashboard to create.
pub fn create_dashboard(
    connection: &mut SqliteConnection,
    dashboard: CreateDashboard,
) -> Result<Dashboard, diesel::result::Error> {
    use crate::schema::dashboards::dsl;

    diesel::insert_into(dsl::dashboards)
        .values(&dashboard)
        .returning(Dashboard::as_returning())
        .get_result(connection)
}

/// Update a dashboard
///
/// This function is used to update a dashboard.
/// It uses the `connection` to update the dashboard in the database.
/// The `dashboard` is the dashboard to update.
pub fn update_dashboard(
    connection: &mut SqliteConnection,
    dashboard: Dashboard,
) -> Result<Dashboard, diesel::result::Error> {
    use crate::schema::dashboards::dsl;

    diesel::update(dsl::dashboards.find(dashboard.id))
        .set((
            dsl::title.eq(dashboard.title),
            dsl::description.eq(dashboard.description),
        ))
        .returning(Dashboard::as_returning())
        .get_result(connection)
}

/// Delete a dashboard
///
/// This function is used to delete a dashboard.
/// It uses the `connection` to delete the dashboard from the database.
/// The `id` is the id of the dashboard to delete.
pub fn delete_dashboard(
    connection: &mut SqliteConnection,
    id: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::dashboards::dsl;

    diesel::delete(dsl::dashboards.find(id))
        .execute(connection)
        .map(|_| ())
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
    fn test_find_dashboard() {
        let mut context = setup_test_environment();
        let dashboard = create_dashboard(
            &mut context.connection,
            CreateDashboard {
                title: "title".to_string(),
                subtitle: "subtitle".to_string(),
                description: "description".to_string(),
                project_id: 1,
            },
        )
        .unwrap();
        let found_dashboard = find_dashboard(&mut context.connection, dashboard.id).unwrap();

        assert_eq!(dashboard.id, found_dashboard.id);
        assert_eq!(dashboard.title, found_dashboard.title);
        assert_eq!(dashboard.description, found_dashboard.description);
    }

    #[test]
    fn test_find_dashboards() {
        let mut context = setup_test_environment();
        let dashboards = find_dashboards(&mut context.connection).unwrap();

        assert_eq!(dashboards.len(), 0);

        create_dashboard(
            &mut context.connection,
            CreateDashboard {
                title: "title 1".to_string(),
                subtitle: "subtitle 1".to_string(),
                description: "description 1".to_string(),
                project_id: 1,
            },
        )
        .unwrap();

        create_dashboard(
            &mut context.connection,
            CreateDashboard {
                title: "title 2".to_string(),
                subtitle: "subtitle 2".to_string(),
                description: "description 2".to_string(),
                project_id: 1,
            },
        )
        .unwrap();

        let dashboards = find_dashboards(&mut context.connection).unwrap();
        assert_eq!(dashboards.len(), 2);
    }

    #[test]
    fn test_update_dashboard() {
        let mut context = setup_test_environment();
        let dashboard = create_dashboard(
            &mut context.connection,
            CreateDashboard {
                title: "title".to_string(),
                subtitle: "subtitle".to_string(),
                description: "description".to_string(),
                project_id: 1,
            },
        )
        .unwrap();

        let updated_dashboard = update_dashboard(
            &mut context.connection,
            Dashboard {
                id: dashboard.id,
                title: "title 2".to_string(),
                subtitle: "subtitle 2".to_string(),
                description: "description 2".to_string(),
                project_id: 1,
            },
        )
        .unwrap();

        assert_eq!(updated_dashboard.id, dashboard.id);
        assert_eq!(updated_dashboard.title, "title 2");
        assert_eq!(updated_dashboard.description, "description 2");
    }

    #[test]
    fn test_delete_dashboard() {
        let mut context = setup_test_environment();
        let dashboard = create_dashboard(
            &mut context.connection,
            CreateDashboard {
                title: "title".to_string(),
                subtitle: "subtitle".to_string(),
                description: "description".to_string(),
                project_id: 1,
            },
        )
        .unwrap();

        delete_dashboard(&mut context.connection, dashboard.id).unwrap();
        let dashboards = find_dashboards(&mut context.connection).unwrap();
        assert_eq!(dashboards.len(), 0);
    }
}
