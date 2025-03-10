use axum::extract::{Query, State};
use axum::{http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::server::state::AppState;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub id: u64,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
}

pub async fn read_users() -> (StatusCode, Json<Vec<User>>) {
    let users = vec![
        User {
            id: 1,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
        },
        User {
            id: 2,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
        },
        User {
            id: 3,
            username: "user3".to_string(),
            email: "user3@example.com".to_string(),
        },
    ];

    (StatusCode::OK, Json(users))
}

pub async fn read_user(params: Query<HashMap<String, String>>) -> (StatusCode, Json<User>) {
    let id = params.get("id").unwrap().parse::<u64>().unwrap();
    let user = User {
        id,
        username: "user".to_string(),
        email: "user@example.com".to_string(),
    };

    (StatusCode::OK, Json(user))
}

pub async fn create_user(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
        email: payload.email,
    };

    (StatusCode::CREATED, Json(user))
}

pub async fn update_user(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: payload.id,
        username: payload.username.unwrap_or("default".to_string()),
        email: payload.email.unwrap_or("default@example.com".to_string()),
    };

    (StatusCode::OK, Json(user))
}
