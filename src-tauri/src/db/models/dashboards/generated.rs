/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

pub type ConnectionType = diesel::sqlite::SqliteConnection;

/// Struct representing a row in table `dashboards`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Identifiable)]
#[diesel(table_name=dashboards, primary_key(id))]
pub struct Dashboards {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `title`
    pub title: String,
    /// Field representing column `subtitle`
    pub subtitle: String,
    /// Field representing column `description`
    pub description: String,
}

/// Create Struct for a row in table `dashboards` for [`Dashboards`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=dashboards)]
pub struct CreateDashboards {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `title`
    pub title: String,
    /// Field representing column `subtitle`
    pub subtitle: String,
    /// Field representing column `description`
    pub description: String,
}

/// Update Struct for a row in table `dashboards` for [`Dashboards`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=dashboards)]
pub struct UpdateDashboards {
    /// Field representing column `title`
    pub title: Option<String>,
    /// Field representing column `subtitle`
    pub subtitle: Option<String>,
    /// Field representing column `description`
    pub description: Option<String>,
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

impl Dashboards {
    /// Insert a new row into `dashboards` with a given [`CreateDashboards`]
    pub fn create(db: &mut ConnectionType, item: &CreateDashboards) -> diesel::QueryResult<Self> {
        use crate::schema::dashboards::dsl::*;

        diesel::insert_into(dashboards).values(item).get_result::<Self>(db)
    }

    /// Get a row from `dashboards`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::dashboards::dsl::*;

        dashboards.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `dashboards`, identified by the primary key with [`UpdateDashboards`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateDashboards) -> diesel::QueryResult<Self> {
        use crate::schema::dashboards::dsl::*;

        diesel::update(dashboards.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `dashboards`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::dashboards::dsl::*;

        diesel::delete(dashboards.filter(id.eq(param_id))).execute(db)
    }
}
