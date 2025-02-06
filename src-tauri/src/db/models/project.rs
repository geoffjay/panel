use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
#[diesel(table_name = crate::schema::projects, primary_key(id))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::projects)]
pub struct CreateProject {
    pub name: String,
    pub description: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq, Default)]
#[diesel(table_name=crate::schema::projects)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub path: Option<String>,
}
