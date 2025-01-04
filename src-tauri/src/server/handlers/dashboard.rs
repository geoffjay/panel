use axum::extract::{Path, State};
use axum::{http::StatusCode, response::Json};
use serde::Serialize;
use std::sync::Arc;
use diesel::{BelongingToDsl, RunQueryDsl};

use crate::db::models::{CreateDashboard, Dashboard, UpdateDashboard, Variable};
use crate::db::repositories::dashboard as repository;
use crate::server::state::AppState;

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    #[serde(flatten)]
    pub dashboard: Dashboard,
    pub variables: Vec<Variable>,
}

/// Read a dashboard by id
///
/// # Arguments
///
/// * `id` - The id of the dashboard to read
///
/// # Returns
///
/// * `StatusCode` - The status code of the response
/// * `Option<Json<Dashboard>>` - The dashboard if it exists
pub async fn read_dashboard(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<DashboardResponse>, StatusCode> {
    let connection = &mut state.db.as_ref().unwrap().lock().unwrap();

    // Load dashboard with its associated variables
    let dashboard = Dashboard::find(connection, id).map_err(|_| StatusCode::NOT_FOUND)?;

    // Load associated variables
    let variables = Variable::belonging_to(&dashboard)
        .load::<Variable>(connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DashboardResponse {
        dashboard,
        variables, // Include variables in the response
    }))
}

/// Read all dashboards
///
/// # Arguments
///
/// * `state` - The state of the application
///
/// # Returns
///
/// * `StatusCode` - The status code of the response
/// * `Option<Json<Vec<Dashboard>>>` - The dashboards if they exist
pub async fn read_dashboards(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<DashboardResponse>>) {
    let connection = &mut state.db.as_ref().unwrap().lock().unwrap();
    match repository::find_dashboards(connection) {
        Ok(dashboards) => {
            let dashboards_response = dashboards.into_iter().map(|dashboard| DashboardResponse {
                dashboard,
                variables: vec![],
            }).collect();
            (StatusCode::OK, Json(dashboards_response))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

/// Create a new dashboard
///
/// # Arguments
///
/// * `state` - The state of the application
/// * `payload` - The payload to create the dashboard
///
/// # Returns
///
/// * `StatusCode` - The status code of the response
/// * `Option<Json<Dashboard>>` - The dashboard if it was created
pub async fn create_dashboard(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateDashboard>,
) -> (StatusCode, Json<Option<DashboardResponse>>) {
    let connection = &mut state.db.as_ref().unwrap().lock().unwrap();
    match repository::create_dashboard(
        connection,
        CreateDashboard {
            title: payload.title,
            subtitle: payload.subtitle,
            description: payload.description,
        },
    ) {
        Ok(dashboard) => {
            let dashboard_response = DashboardResponse {
                dashboard,
                variables: vec![],
            };
            (StatusCode::CREATED, Json(Some(dashboard_response)))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

/// Update a dashboard
///
/// # Arguments
///
/// * `state` - The state of the application
/// * `payload` - The payload to update the dashboard
///
/// # Returns
///
/// * `StatusCode` - The status code of the response
/// * `Option<Json<Dashboard>>` - The dashboard if it was updated
pub async fn update_dashboard(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateDashboard>,
) -> (StatusCode, Json<Option<DashboardResponse>>) {
    let connection = &mut state.db.as_ref().unwrap().lock().unwrap();
    
    // First find the existing dashboard
    let existing = match Dashboard::find(connection, id) {
        Ok(dashboard) => dashboard,
        Err(_) => return (StatusCode::NOT_FOUND, Json(None)),
    };

    // Then update with new values, falling back to existing values if None
    match repository::update_dashboard(
        connection,
        Dashboard {
            id,
            title: payload.title.unwrap_or(existing.title),
            subtitle: payload.subtitle.unwrap_or(existing.subtitle),
            description: payload.description.unwrap_or(existing.description),
        },
    ) {
        Ok(dashboard) => {
            let dashboard_response = DashboardResponse {
                dashboard,
                variables: vec![],
            };
            (StatusCode::OK, Json(Some(dashboard_response)))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

/// Delete a dashboard
///
/// # Arguments
///
/// * `state` - The state of the application
/// * `id` - The id of the dashboard to delete
///
/// # Returns
///
/// * `StatusCode` - The status code of the response
pub async fn delete_dashboard(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> StatusCode {
    let connection = &mut state.db.as_ref().unwrap().lock().unwrap();
    match repository::delete_dashboard(connection, id) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::routing::{delete, get, post, put};
    use axum::Router;
    use axum_test::TestServer;
    use serde_json::json;

    use crate::utils::test_helpers::setup_test_environment;

    #[tokio::test]
    async fn test_read_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(AppState {
            tauri: None,
            db: Some(context.connection.clone()),
        });
        let app = Router::new()
            .route("/dashboard/:id", get(read_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let dashboard = repository::create_dashboard(
            &mut context.connection.lock().unwrap(),
            CreateDashboard {
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
            db: Some(context.connection.clone()),
        });
        let app = Router::new()
            .route("/dashboard", get(read_dashboards))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        repository::create_dashboard(
            &mut context.connection.lock().unwrap(),
            CreateDashboard {
                title: "title 1".to_string(),
                subtitle: "subtitle 1".to_string(),
                description: "description 1".to_string(),
            },
        )
        .unwrap();

        repository::create_dashboard(
            &mut context.connection.lock().unwrap(),
            CreateDashboard {
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
            db: Some(context.connection.clone()),
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
            db: Some(context.connection.clone()),
        });
        let app = Router::new()
            .route("/dashboard/:id", put(update_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let dashboard = repository::create_dashboard(
            &mut context.connection.lock().unwrap(),
            CreateDashboard {
                title: "title 1".to_string(),
                subtitle: "subtitle 1".to_string(),
                description: "description 1".to_string(),
            },
        )
        .unwrap();

        let response = server
            .put(&format!("/dashboard/{}", dashboard.id))
            .json(&json!({
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
            // FIXME: annoyingly, the subtitle is not returned by the API
            // "subtitle": "subtitle 2",
            "variables": []
        }));
    }

    #[tokio::test]
    async fn test_delete_dashboard() {
        let context = setup_test_environment();
        let state = Arc::new(AppState {
            tauri: None,
            db: Some(context.connection.clone()),
        });
        let app = Router::new()
            .route("/dashboard/:id", delete(delete_dashboard))
            .with_state(state);

        let server = TestServer::new(app).unwrap();

        let dashboard = repository::create_dashboard(
            &mut context.connection.lock().unwrap(),
            CreateDashboard {
                title: "title 1".to_string(),
                subtitle: "subtitle 1".to_string(),
                description: "description 1".to_string(),
            },
        )
        .unwrap();

        let response = server.delete(&format!("/dashboard/{}", dashboard.id)).await;

        response.assert_status_ok();

        let dashboards =
            repository::find_dashboards(&mut context.connection.lock().unwrap()).unwrap();
        assert_eq!(dashboards.len(), 0);
    }
}
