use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::ConnectionType;

/// Dashboard model
///
/// This model is used to represent a dashboard.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Identifiable,
    Selectable,
    Queryable,
    QueryableByName,
)]
#[diesel(table_name = crate::schema::dashboards, primary_key(id))]
pub struct Dashboard {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub description: String,
}

/// Model to create a new dashboard
///
/// This model is used to represent a new dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::dashboards)]
pub struct CreateDashboard {
    pub title: String,
    pub subtitle: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq, Default)]
#[diesel(table_name=crate::schema::dashboards)]
pub struct UpdateDashboard {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}

// TODO: Implement pagination in the db mod
// #[derive(Debug, Serialize)]
// pub struct PaginatedResult<T> {
//     pub items: Vec<T>,
//     pub total_items: i64,
//     pub page: i64,
//     pub page_size: i64,
//     pub num_pages: i64,
// }

impl Dashboard {
    pub fn new(title: String, subtitle: String, description: String) -> Self {
        Self {
            id: 0,
            title,
            subtitle,
            description,
        }
    }

    pub fn find(db: &mut ConnectionType, id: i32) -> QueryResult<Self> {
        use crate::schema::dashboards::dsl;

        dsl::dashboards.find(id).first(db)
    }

    pub fn find_all(db: &mut ConnectionType) -> QueryResult<Vec<Self>> {
        use crate::schema::dashboards::dsl;

        dsl::dashboards.load::<Self>(db)
    }

    pub fn create(db: &mut ConnectionType, item: &CreateDashboard) -> QueryResult<Self> {
        use crate::schema::dashboards::dsl;

        diesel::insert_into(dsl::dashboards)
            .values(item)
            .get_result::<Self>(db)
    }

    pub fn update(
        db: &mut ConnectionType,
        param_id: i32,
        item: &UpdateDashboard,
    ) -> diesel::QueryResult<Self> {
        use crate::schema::dashboards::dsl;

        diesel::update(dsl::dashboards.filter(dsl::id.eq(param_id)))
            .set(item)
            .returning(Dashboard::as_returning())
            .get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::dashboards::dsl;

        diesel::delete(dsl::dashboards.filter(dsl::id.eq(param_id))).execute(db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_helpers::setup_test_environment;

    #[test]
    fn test_dashboard_serialization() {
        let dashboard = Dashboard::new(
            "Test Dashboard".to_string(),
            "This is a test dashboard".to_string(),
            "This is a test dashboard".to_string(),
        );

        let serialized = serde_json::to_string(&dashboard).unwrap();
        let deserialized: Dashboard = serde_json::from_str(&serialized).unwrap();

        assert_eq!(dashboard, deserialized);
    }

    #[test]
    fn test_dashboard_find() {
        let context = setup_test_environment();
        let dashboard = Dashboard::create(
            &mut context.connection.lock().unwrap(),
            &CreateDashboard {
                title: "Test Dashboard".to_string(),
                subtitle: "This is a test dashboard".to_string(),
                description: "This is a test dashboard".to_string(),
            },
        )
        .unwrap();

        let found_dashboard =
            Dashboard::find(&mut context.connection.lock().unwrap(), dashboard.id).unwrap();
        assert_eq!(found_dashboard, dashboard);
    }

    #[test]
    fn test_dashboard_find_all() {
        let context = setup_test_environment();
        let dashboards = Dashboard::find_all(&mut context.connection.lock().unwrap()).unwrap();

        assert!(dashboards.is_empty());

        let _ = Dashboard::create(
            &mut context.connection.lock().unwrap(),
            &CreateDashboard {
                title: "Test Dashboard".to_string(),
                subtitle: "This is a test dashboard".to_string(),
                description: "This is a test dashboard".to_string(),
            },
        )
        .unwrap();
        let dashboards = Dashboard::find_all(&mut context.connection.lock().unwrap()).unwrap();

        assert!(!dashboards.is_empty());
        assert_eq!(dashboards.len(), 1);
    }

    #[test]
    fn test_dashboard_create() {
        let context = setup_test_environment();
        let dashboard = Dashboard::create(
            &mut context.connection.lock().unwrap(),
            &CreateDashboard {
                title: "Test Dashboard".to_string(),
                subtitle: "This is a test dashboard".to_string(),
                description: "This is a test dashboard".to_string(),
            },
        )
        .unwrap();

        assert_eq!(dashboard.title, "Test Dashboard");
        assert_eq!(dashboard.subtitle, "This is a test dashboard");
        assert_eq!(dashboard.description, "This is a test dashboard");

        let found_dashboard =
            Dashboard::find(&mut context.connection.lock().unwrap(), dashboard.id).unwrap();

        assert_eq!(found_dashboard, dashboard);
    }

    #[test]
    fn test_dashboard_update() {
        let context = setup_test_environment();
        let dashboard = Dashboard::create(
            &mut context.connection.lock().unwrap(),
            &CreateDashboard {
                title: "Test Dashboard".to_string(),
                subtitle: "This is a test dashboard".to_string(),
                description: "This is a test dashboard".to_string(),
            },
        )
        .unwrap();

        let updated_dashboard = Dashboard::update(
            &mut context.connection.lock().unwrap(),
            dashboard.id,
            &UpdateDashboard {
                title: Some("Updated Test Dashboard".to_string()),
                subtitle: Some("This is an updated test dashboard".to_string()),
                description: Some("This is an updated test dashboard".to_string()),
            },
        )
        .unwrap();

        assert_eq!(updated_dashboard.title, "Updated Test Dashboard");
        assert_eq!(
            updated_dashboard.subtitle,
            "This is an updated test dashboard"
        );
        assert_eq!(
            updated_dashboard.description,
            "This is an updated test dashboard"
        );
    }

    #[test]
    fn test_dashboard_delete() {
        let context = setup_test_environment();
        let dashboard = Dashboard::create(
            &mut context.connection.lock().unwrap(),
            &CreateDashboard {
                title: "Test Dashboard".to_string(),
                subtitle: "This is a test dashboard".to_string(),
                description: "This is a test dashboard".to_string(),
            },
        )
        .unwrap();

        let deleted_dashboard =
            Dashboard::delete(&mut context.connection.lock().unwrap(), dashboard.id).unwrap();
        assert_eq!(deleted_dashboard, 1);

        let dashboards = Dashboard::find_all(&mut context.connection.lock().unwrap()).unwrap();
        assert!(dashboards.is_empty());
    }

    // #[test]
    // fn test_dashboard_pagination() {
    //     use crate::schema::dashboards::dsl;

    //     let mut context = setup_test_environment();

    //     let _ = Dashboard::create(
    //         &mut context.connection,
    //         &CreateDashboard {
    //             title: "Test Dashboard 1".to_string(),
    //             subtitle: "This is a test dashboard".to_string(),
    //             description: "This is a test dashboard".to_string(),
    //         },
    //     )
    //     .unwrap();

    //     let _ = Dashboard::create(
    //         &mut context.connection,
    //         &CreateDashboard {
    //             title: "Test Dashboard 2".to_string(),
    //             subtitle: "This is a test dashboard".to_string(),
    //             description: "This is a test dashboard".to_string(),
    //         },
    //     )
    //     .unwrap();

    //     let pagination_result = Dashboard::find_all(&mut context.connection).unwrap();

    //     assert_eq!(pagination_result.items.len(), 1);
    //     assert_eq!(pagination_result.total_items, 1);
    //     assert_eq!(pagination_result.page, 0);
    //     assert_eq!(pagination_result.page_size, 10);
    //     assert_eq!(pagination_result.num_pages, 1);
    // }
}
