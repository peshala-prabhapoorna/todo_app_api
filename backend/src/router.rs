use axum::{routing::get, Router};

use crate::routes::greeting::greeting;

pub async fn create_routes() -> Router {
    Router::new()
        .route("/api/v1/greeting", get(greeting))
}