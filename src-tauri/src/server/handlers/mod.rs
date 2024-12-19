pub mod dashboard;
pub mod user;

pub use self::dashboard::*;
pub use self::user::*;

use axum::{http::StatusCode, response::Json};

pub async fn root() -> (StatusCode, Json<&'static str>) {
    (StatusCode::OK, Json("Hello, world!"))
}
