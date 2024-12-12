pub mod handlers;
pub mod state;

use std::io;
use std::sync::Arc;
use axum::{Router, routing::{get, post, patch, delete}};
use http::Method;
use tauri::AppHandle;
use tokio;
use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;

use crate::server::handlers::{root, create_user, read_users, read_user, update_user, delete_user};
use crate::server::state::AppState;

#[tokio::main]
pub async fn init(tauri: AppHandle) -> io::Result<()> {
    let shared_state = Arc::new(AppState { tauri });
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user).with_state(shared_state))
        .route("/users", get(read_users))
        .route("/users/:id", get(read_user))
        .route("/users", patch(update_user).with_state(shared_state))
        .route("/users", delete(delete_user).with_state(shared_state))
        .layer(ServiceBuilder::new().layer(cors_layer));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}
