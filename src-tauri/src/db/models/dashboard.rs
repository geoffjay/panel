use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Dashboard model
/// 
/// This model is used to represent a dashboard.
#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::dashboards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Dashboard {
    pub id: i32,
    pub title: String,
    pub description: String,
}

/// New dashboard model
/// 
/// This model is used to represent a new dashboard.
#[derive(Insertable)]
#[diesel(table_name = crate::schema::dashboards)]
pub struct NewDashboard {
    pub title: String,
    pub description: String,
}
