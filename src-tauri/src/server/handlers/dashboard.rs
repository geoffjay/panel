use axum::extract::{Path, State};
use axum::{http::StatusCode, response::Json};
use serde::Deserialize;
use std::sync::Arc;

use crate::db::models::{Dashboard, NewDashboard};
use crate::db::repositories::dashboard as repository;
use crate::server::state::AppState;

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

pub async fn read_dashboard(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Dashboard>) {
    let db = &mut state.db.as_ref().unwrap().lock().unwrap();
    let dashboard = repository::find_dashboard(db, id);

    (StatusCode::OK, Json(dashboard.unwrap()))
}

pub async fn read_dashboards(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<Dashboard>>) {
    let db = &mut state.db.as_ref().unwrap().lock().unwrap();
    let dashboards = repository::find_dashboards(db);

    (StatusCode::OK, Json(dashboards.unwrap()))
}

pub async fn create_dashboard(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateDashboard>,
) -> (StatusCode, Json<Dashboard>) {
    let db = &mut state.db.as_ref().unwrap().lock().unwrap();
    let dashboard = repository::create_dashboard(
        db,
        NewDashboard {
            title: payload.title,
            subtitle: "subtitle".to_string(),
            description: payload.description,
        },
    );

    (StatusCode::CREATED, Json(dashboard.unwrap()))
}

pub async fn update_dashboard(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateDashboard>,
) -> (StatusCode, Json<Dashboard>) {
    let db = &mut state.db.as_ref().unwrap().lock().unwrap();
    let dashboard = repository::update_dashboard(
        db,
        Dashboard {
            id: payload.id,
            title: payload.title.unwrap(),
            subtitle: "subtitle".to_string(),
            description: payload.description.unwrap(),
        },
    );

    (StatusCode::OK, Json(dashboard.unwrap()))
}

pub async fn delete_dashboard(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> StatusCode {
    let db = &mut state.db.as_ref().unwrap().lock().unwrap();
    repository::delete_dashboard(db, id);

    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::{delete, get, post, put};
    use axum::Router;
    use axum_test::TestServer;
    use diesel::prelude::*;
    use diesel_migrations::MigrationHarness;
    use serde_json::json;
    use std::sync::Mutex;

    use crate::MIGRATIONS;

    struct TestContext {
        db: Arc<Mutex<SqliteConnection>>,
    }

    fn setup_test_environment() -> TestContext {
        let database_url = ":memory:".to_string();
        let mut connection = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        connection.run_pending_migrations(MIGRATIONS).unwrap();

        TestContext {
            db: Arc::new(Mutex::new(connection)),
        }
    }

    #[tokio::test]
    async fn test_read_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(AppState {
            tauri: None,
            db: Some(context.db.clone()),
        });
        let app = Router::new()
            .route("/dashboard/:id", get(read_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let dashboard = repository::create_dashboard(
            &mut context.db.lock().unwrap(),
            NewDashboard {
                title: "title".to_string(),
                subtitle: "subtitle".to_string(),
                description: "description".to_string(),
            },
        )
        .unwrap();

        let response = server.get(&format!("/dashboard/{}", dashboard.id)).await;

        response.assert_status_ok();
        response.assert_json_contains(&json!({
            "id": dashboard.id,
            "title": dashboard.title,
            "description": dashboard.description,
            "variables": []
        }));
    }

    #[tokio::test]
    async fn test_read_dashboards() {
        let context = setup_test_environment();
        let state = Arc::new(AppState {
            tauri: None,
            db: Some(context.db.clone()),
        });
        let app = Router::new()
            .route("/dashboard", get(read_dashboards))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        repository::create_dashboard(
            &mut context.db.lock().unwrap(),
            NewDashboard {
                title: "title 1".to_string(),
                subtitle: "subtitle 1".to_string(),
                description: "description 1".to_string(),
            },
        )
        .unwrap();

        repository::create_dashboard(
            &mut context.db.lock().unwrap(),
            NewDashboard {
                title: "title 2".to_string(),
                subtitle: "subtitle 2".to_string(),
                description: "description 2".to_string(),
            },
        )
        .unwrap();

        let response = server.get("/dashboard").await;

        response.assert_status_ok();
        response.assert_json_contains(&json!([
            { "id": 1, "title": "title 1", "description": "description 1", "variables": [] },
            { "id": 2, "title": "title 2", "description": "description 2", "variables": [] },
        ]));
    }

    #[tokio::test]
    async fn test_create_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(AppState {
            tauri: None,
            db: Some(context.db.clone()),
        });
        let app = Router::new()
            .route("/dashboard", post(create_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/dashboard")
            .json(&json!({
                "title": "title",
                "description": "description",
                "subtitle": "subtitle"
            }))
            .await;

        response.assert_status(StatusCode::CREATED);
        response.assert_json_contains(&json!({
            "id": 1,
            "title": "title",
            "description": "description",
            "variables": []
        }));
    }

    #[tokio::test]
    async fn test_update_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(AppState {
            tauri: None,
            db: Some(context.db.clone()),
        });
        let app = Router::new()
            .route("/dashboard", put(update_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let dashboard = repository::create_dashboard(
            &mut context.db.lock().unwrap(),
            NewDashboard {
                title: "title 1".to_string(),
                subtitle: "subtitle 1".to_string(),
                description: "description 1".to_string(),
            },
        )
        .unwrap();

        let response = server
            .put("/dashboard")
            .json(&json!({
                "id": dashboard.id,
                "title": "title 2",
                "description": "description 2",
                "subtitle": "subtitle 2",
            }))
            .await;

        response.assert_status_ok();
        response.assert_json_contains(&json!({
            "id": dashboard.id,
            "title": "title 2",
            "description": "description 2",
            "subtitle": "subtitle 2",
        }));
    }

    #[tokio::test]
    async fn test_delete_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(AppState {
            tauri: None,
            db: Some(context.db.clone()),
        });
        let app = Router::new()
            .route("/dashboard/:id", delete(delete_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let dashboard = repository::create_dashboard(
            &mut context.db.lock().unwrap(),
            NewDashboard {
                title: "title 1".to_string(),
                subtitle: "subtitle 1".to_string(),
                description: "description 1".to_string(),
            },
        )
        .unwrap();

        let response = server.delete(&format!("/dashboard/{}", dashboard.id)).await;

        response.assert_status_ok();

        let dashboards = repository::find_dashboards(&mut context.db.lock().unwrap()).unwrap();
        assert_eq!(dashboards.len(), 0);
    }
}
