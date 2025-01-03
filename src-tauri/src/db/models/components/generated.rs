/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::models::dashboards::Dashboards;
use crate::schema::*;

pub type ConnectionType = diesel::sqlite::SqliteConnection;

/// Struct representing a row in table `components`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName, diesel::Associations, diesel::Identifiable)]
#[diesel(table_name=components, primary_key(id), belongs_to(Dashboards, foreign_key=dashboard_id))]
pub struct Components {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `dashboard_id`
    pub dashboard_id: Option<i32>,
    /// Field representing column `parent_id`
    pub parent_id: Option<i32>,
    /// Field representing column `type`
    pub comp_type: String,
}

/// Create Struct for a row in table `components` for [`Components`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=components)]
pub struct CreateComponents {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `dashboard_id`
    pub dashboard_id: Option<i32>,
    /// Field representing column `parent_id`
    pub parent_id: Option<i32>,
    /// Field representing column `type`
    pub comp_type: String,
}

/// Update Struct for a row in table `components` for [`Components`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=components)]
pub struct UpdateComponents {
    /// Field representing column `dashboard_id`
    pub dashboard_id: Option<Option<i32>>,
    /// Field representing column `parent_id`
    pub parent_id: Option<Option<i32>>,
    /// Field representing column `type`
    pub comp_type: Option<String>,
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

impl Components {
    /// Insert a new row into `components` with a given [`CreateComponents`]
    pub fn create(db: &mut ConnectionType, item: &CreateComponents) -> diesel::QueryResult<Self> {
        use crate::schema::components::dsl::*;

        diesel::insert_into(components).values(item).get_result::<Self>(db)
    }

    /// Get a row from `components`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<Self> {
        use crate::schema::components::dsl::*;

        components.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Update a row in `components`, identified by the primary key with [`UpdateComponents`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateComponents) -> diesel::QueryResult<Self> {
        use crate::schema::components::dsl::*;

        diesel::update(components.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `components`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> diesel::QueryResult<usize> {
        use crate::schema::components::dsl::*;

        diesel::delete(components.filter(id.eq(param_id))).execute(db)
    }
}
