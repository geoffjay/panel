use axum::{response::Json, http::StatusCode};
use axum::extract::{State, Path};
use diesel::prelude::*;
use serde::Deserialize;
use std::sync::Arc;

use crate::server::state::AppState;
use crate::models::Dashboard;

#[derive(Deserialize)]
pub struct CreateDashboard {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateDashboard {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteDashboard {
    pub id: i32,
}

#[axum::debug_handler]
pub async fn read_dashboard(
    Path(dashboard_id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Dashboard>) {
    use crate::schema::dashboards::dsl::*;

    let db = &mut *state.db.as_ref().unwrap().lock().unwrap();
    let dashboard: Dashboard = dashboards
        .find(dashboard_id)
        .first(db)
        .expect("Error loading dashboard");

    (
        StatusCode::OK,
        Json(Dashboard {
            id: dashboard.id,
            title: dashboard.title,
            description: dashboard.description,
        })
    )
}

pub async fn read_dashboards() -> (StatusCode, Json<Vec<Dashboard>>) {
    (StatusCode::OK, Json(vec![
        Dashboard { id: 1, title: "title".to_string(), description: "description".to_string() },
        Dashboard { id: 2, title: "title".to_string(), description: "description".to_string() },
        Dashboard { id: 3, title: "title".to_string(), description: "description".to_string() },
    ]))
}

pub async fn create_dashboard(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<CreateDashboard>,
) -> (StatusCode, Json<Dashboard>) {
    (StatusCode::CREATED, Json(Dashboard { id: 1, title: payload.title, description: payload.description }))
}

pub async fn update_dashboard(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<UpdateDashboard>,
) -> (StatusCode, Json<Dashboard>) {
    (StatusCode::OK, Json(Dashboard { id: payload.id, title: payload.title.unwrap(), description: payload.description.unwrap() }))
}

pub async fn delete_dashboard(
    State(_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Dashboard>) {
    (StatusCode::OK, Json(Dashboard { id, title: "title".to_string(), description: "description".to_string() }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use axum::Router;
    use axum::routing::{get, post, put, delete};
    use axum_test::TestServer;
    use diesel::SqliteConnection;
    use diesel_migrations::MigrationHarness;
    use serde_json::json;

    use crate::db::establish_test_connection;
    use crate::MIGRATIONS;

    struct TestContext {
        connection: SqliteConnection,
    }

    fn setup_test_environment() -> TestContext {
        let mut connection: SqliteConnection = establish_test_connection();
        connection.run_pending_migrations(MIGRATIONS).unwrap();

        TestContext { connection }
    }

    #[tokio::test]
    async fn test_read_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(AppState{ tauri: None, db: Some(Mutex::new(context.connection)) });
        let app = Router::new()
            .route(&"/dashboard/:id", get(read_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let response = server
            .get("/dashboard/1")
            .await;

        response.assert_status_ok();
        response.assert_json_contains(&json!({
            "id": 1,
            "title": "title",
            "description": "description"
        }));
    }

    #[tokio::test]
    async fn test_read_dashboards() {
        let app = Router::new()
            .route(&"/dashboard", get(read_dashboards));

        let server = TestServer::new(app).unwrap();

        let response = server
            .get("/dashboard")
            .await;

        response.assert_status_ok();
        response.assert_json_contains(&json!([
            { "id": 1, "title": "title", "description": "description" },
            { "id": 2, "title": "title", "description": "description" },
            { "id": 3, "title": "title", "description": "description" },
        ]));
    }

    #[tokio::test]
    async fn test_create_dashboard() {
        let state = Arc::new(AppState{ tauri: None, db: None });
        let app = Router::new()
            .route(&"/dashboard", post(create_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/dashboard")
            .json(&json!({
                "title": "title",
                "description": "description"
            }))
            .await;

        response.assert_status(StatusCode::CREATED);
        response.assert_json_contains(&json!({
            "id": 1,
            "title": "title",
            "description": "description"
        }));
    }

    #[tokio::test]
    async fn test_update_dashboard() {
        let state = Arc::new(AppState{ tauri: None, db: None });
        let app = Router::new()
            .route(&"/dashboard", put(update_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let response = server
            .put("/dashboard")
            .json(&json!({
                "id": 1,
                "title": "title",
                "description": "description"
            }))
            .await;

        response.assert_status_ok();
        response.assert_json_contains(&json!({
            "id": 1,
            "title": "title",
            "description": "description"
        }));
    }

    #[tokio::test]
    async fn test_delete_dashboard() {
        let state = Arc::new(AppState{ tauri: None, db: None });
        let app = Router::new()
            .route(&"/dashboard/:id", delete(delete_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let response = server
            .delete("/dashboard/1")
            .await;

        response.assert_status_ok();
        response.assert_json_contains(&json!({
            "id": 1,
            "title": "title",
            "description": "description"
        }));
    }
}
