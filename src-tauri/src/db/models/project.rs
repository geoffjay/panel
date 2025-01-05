use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Identifiable, Selectable, Queryable, QueryableByName)]
#[diesel(table_name = crate::schema::projects, primary_key(id))]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::projects)]
pub struct CreateProject {
    pub title: String,
    pub subtitle: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq, Default)]
#[diesel(table_name=crate::schema::projects)]
pub struct UpdateProject {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}
