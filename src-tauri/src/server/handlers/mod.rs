pub mod user;
pub mod dashboard;

pub use self::user::*;
pub use self::dashboard::*;

use axum::{response::Json, http::StatusCode};

pub async fn root() -> (StatusCode, Json<&'static str>) {
    (StatusCode::OK, Json("Hello, world!"))
}
