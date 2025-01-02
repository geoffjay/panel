use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Dashboard model
///
/// This model is used to represent a dashboard.
#[derive(Queryable, Selectable, Deserialize, Identifiable, Serialize, PartialEq, Debug)]
#[diesel(table_name = crate::schema::dashboards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Dashboard {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub description: String,
}

/// New dashboard model
///
/// This model is used to represent a new dashboard.
#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::dashboards)]
pub struct NewDashboard {
    pub title: String,
    pub subtitle: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_serialization() {
        let dashboard = Dashboard {
            id: 1,
            title: "Test Dashboard".to_string(),
            subtitle: "This is a test dashboard".to_string(),
            description: "This is a test dashboard".to_string(),
        };

        let serialized = serde_json::to_string(&dashboard).unwrap();
        let deserialized: Dashboard = serde_json::from_str(&serialized).unwrap();

        assert_eq!(dashboard, deserialized);
    }
}
