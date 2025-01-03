/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::dashboards::Dashboards;
use crate::schema::*;

pub type ConnectionType = diesel::sqlite::SqliteConnection;

/// Struct representing a row in table `variables`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Associations, diesel::Identifiable)]
#[diesel(table_name=variables, primary_key(id), belongs_to(Dashboards, foreign_key=dashboard_id))]
pub struct Variables {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `ref_id`
    pub ref_id: Option<String>,
    /// Field representing column `default`
    pub default: Option<serde_json::Value>,
    /// Field representing column `value`
    pub value: serde_json::Value,
    /// Field representing column `dashboard_id`
    pub dashboard_id: i32,
}

/// Create Struct for a row in table `variables` for [`Variables`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=variables)]
pub struct CreateVariables {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `ref_id`
    pub ref_id: Option<String>,
    /// Field representing column `default`
    pub default: Option<serde_json::Value>,
    /// Field representing column `value`
    pub value: serde_json::Value,
    /// Field representing column `dashboard_id`
    pub dashboard_id: i32,
}

/// Update Struct for a row in table `variables` for [`Variables`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=variables)]
pub struct UpdateVariables {
    /// Field representing column `ref_id`
    pub ref_id: Option<Option<String>>,
    /// Field representing column `default`
    pub default: Option<Option<serde_json::Value>>,
    /// Field representing column `value`
    pub value: Option<serde_json::Value>,
    /// Field representing column `dashboard_id`
    pub dashboard_id: Option<i32>,
}

/// Result of a `.paginate` function
#[derive(Debug, serde::Serialize)]
pub struct PaginationResult<T> {
    /// Resulting items that are from the current page
    pub items: Vec<T>,
    /// The count of total items there are
    pub total_items: i64,
    /// Current page, 0-based index
    pub page: i64,
    /// Size of a page
    pub page_size: i64,
    /// Number of total possible pages, given the `page_size` and `total_items`
    pub num_pages: i64,
}

impl Variables {
    /// Insert a new row into `variables` with a given [`CreateVariables`]
    pub fn create(db: &mut ConnectionType, item: &CreateVariables) -> diesel::QueryResult<Self> {
        use crate::schema::variables::dsl::*;

        diesel::insert_into(variables).values(item).get_result::<Self>(db)
    }

    /// Get a row from `variables`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::variables::dsl::*;

        variables.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `variables`, identified by the primary key with [`UpdateVariables`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateVariables) -> diesel::QueryResult<Self> {
        use crate::schema::variables::dsl::*;

        diesel::update(variables.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `variables`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::variables::dsl::*;

        diesel::delete(variables.filter(id.eq(param_id))).execute(db)
    }
}
