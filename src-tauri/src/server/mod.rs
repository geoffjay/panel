pub mod handlers;
pub mod state;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use diesel::SqliteConnection;
use http::Method;
use std::io;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tokio;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::server::handlers::{
    create_dashboard, create_user, delete_dashboard, delete_user, read_dashboard, read_dashboards,
    read_user, read_users, root, update_dashboard, update_user,
};
use crate::server::state::AppState;

#[tokio::main]
pub async fn init(tauri: AppHandle, db: SqliteConnection) -> io::Result<()> {
    let shared_state = Arc::new(AppState {
        tauri: Some(tauri),
        db: Some(Mutex::new(db).into()),
    });
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user).with_state(shared_state.clone()))
        .route("/users", get(read_users))
        .route("/users/:id", get(read_user))
        .route(
            "/users",
            patch(update_user).with_state(shared_state.clone()),
        )
        .route(
            "/users",
            delete(delete_user).with_state(shared_state.clone()),
        )
        .route(
            "/dashboards",
            get(read_dashboards).with_state(shared_state.clone()),
        )
        .route(
            "/dashboards/:id",
            get(read_dashboard).with_state(shared_state.clone()),
        )
        .route(
            "/dashboards",
            post(create_dashboard).with_state(shared_state.clone()),
        )
        .route(
            "/dashboards",
            patch(update_dashboard).with_state(shared_state.clone()),
        )
        .route(
            "/dashboards",
            delete(delete_dashboard).with_state(shared_state.clone()),
        )
        .layer(ServiceBuilder::new().layer(cors_layer));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}
