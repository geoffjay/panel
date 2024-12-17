use axum::{response::Json, http::StatusCode};
use axum::extract::{State, Path};
use serde::Deserialize;
use std::sync::Arc;

use crate::server::state::AppState;
use crate::models::{Dashboard, NewDashboard};
use crate::db::{
    get_dashboard,
    get_dashboards,
    create_dashboard as create_dashboard_db,
    update_dashboard as update_dashboard_db,
    delete_dashboard as delete_dashboard_db
};

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

pub async fn read_dashboard(
    Path(dashboard_id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Dashboard>) {
    let db = &mut *state.db.as_ref().unwrap().lock().unwrap();
    let dashboard = get_dashboard(db, dashboard_id);

    (
        StatusCode::OK,
        Json(dashboard)
    )
}

pub async fn read_dashboards(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<Dashboard>>) {
    let db = &mut *state.db.as_ref().unwrap().lock().unwrap();
    let dashboards = get_dashboards(db);

    (StatusCode::OK, Json(dashboards))
}

pub async fn create_dashboard(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateDashboard>,
) -> (StatusCode, Json<Dashboard>) {
    let db = &mut *state.db.as_ref().unwrap().lock().unwrap();
    let dashboard = create_dashboard_db(
        db,
        NewDashboard { title: payload.title, description: payload.description }
    );

    (StatusCode::CREATED, Json(dashboard))
}

pub async fn update_dashboard(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateDashboard>,
) -> (StatusCode, Json<Dashboard>) {
    let db = &mut *state.db.as_ref().unwrap().lock().unwrap();
    let dashboard = update_dashboard_db(
        db,
        Dashboard {
            id: payload.id,
            title: payload.title.unwrap(),
            description: payload.description.unwrap()
        }
    );

    (StatusCode::OK, Json(dashboard))
}

pub async fn delete_dashboard(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> StatusCode {
    let db = &mut *state.db.as_ref().unwrap().lock().unwrap();
    delete_dashboard_db(db, id);

    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use axum::Router;
    use axum::routing::{get, post, put, delete};
    use axum_test::TestServer;
    use diesel::prelude::*;
    use diesel_migrations::MigrationHarness;
    use serde_json::json;

    use crate::models::NewDashboard;
    use crate::db::create_dashboard as create_dashboard_db;
    use crate::MIGRATIONS;

    struct TestContext {
        db: Arc<Mutex<SqliteConnection>>,
    }

    fn setup_test_environment() -> TestContext {
        let database_url = ":memory:".to_string();
        let mut connection = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        connection.run_pending_migrations(MIGRATIONS).unwrap();

        TestContext { db: Arc::new(Mutex::new(connection)) }
    }

    #[tokio::test]
    async fn test_read_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(
            AppState{
                tauri: None,
                db: Some(context.db.clone())
            }
        );
        let app = Router::new()
            .route(&"/dashboard/:id", get(read_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let dashboard = create_dashboard_db(
            &mut *context.db.lock().unwrap(),
            NewDashboard {
                title: "title".to_string(),
                description: "description".to_string()
            }
        );

        let response = server
            .get(&format!("/dashboard/{}", dashboard.id))
            .await;

        response.assert_status_ok();
        response.assert_json_contains(&json!({
            "id": dashboard.id,
            "title": dashboard.title,
            "description": dashboard.description
        }));
    }

    #[tokio::test]
    async fn test_read_dashboards() {
        let context = setup_test_environment();
        let state = Arc::new(
            AppState{
                tauri: None,
                db: Some(context.db.clone())
            }
        );
        let app = Router::new()
            .route(&"/dashboard", get(read_dashboards))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        create_dashboard_db(
            &mut *context.db.lock().unwrap(),
            NewDashboard {
                title: "title 1".to_string(),
                description: "description 1".to_string()
            }
        );

        create_dashboard_db(
            &mut *context.db.lock().unwrap(),
            NewDashboard {
                title: "title 2".to_string(),
                description: "description 2".to_string()
            }
        );

        let response = server
            .get("/dashboard")
            .await;

        response.assert_status_ok();
        response.assert_json_contains(&json!([
            { "id": 1, "title": "title 1", "description": "description 1" },
            { "id": 2, "title": "title 2", "description": "description 2" },
        ]));
    }

    #[tokio::test]
    async fn test_create_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(
            AppState{
                tauri: None,
                db: Some(context.db.clone()),
            }
        );
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
        let context = setup_test_environment();
        let state = Arc::new(
            AppState{
                tauri: None,
                db: Some(context.db.clone())
            }
        );
        let app = Router::new()
            .route(&"/dashboard", put(update_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let dashboard = create_dashboard_db(
            &mut *context.db.lock().unwrap(),
            NewDashboard {
                title: "title 1".to_string(),
                description: "description 1".to_string()
            }
        );

        let response = server
            .put("/dashboard")
            .json(&json!({
                "id": dashboard.id,
                "title": "title 2",
                "description": "description 2"
            }))
            .await;

        response.assert_status_ok();
        response.assert_json_contains(&json!({
            "id": dashboard.id,
            "title": "title 2",
            "description": "description 2"
        }));
    }

    #[tokio::test]
    async fn test_delete_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(
            AppState{
                tauri: None,
                db: Some(context.db.clone())
            }
        );
        let app = Router::new()
            .route(&"/dashboard/:id", delete(delete_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let dashboard = create_dashboard_db(
            &mut *context.db.lock().unwrap(),
            NewDashboard {
                title: "title 1".to_string(),
                description: "description 1".to_string()
            }
        );

        let response = server
            .delete(&format!("/dashboard/{}", dashboard.id))
            .await;

        response.assert_status_ok();

        let dashboards = get_dashboards(&mut *context.db.lock().unwrap());
        assert_eq!(dashboards.len(), 0);
    }
}
