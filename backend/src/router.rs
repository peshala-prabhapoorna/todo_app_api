use axum::{routing::get, Router};

use crate::{app_state::AppState, routes::greeting::greeting};

pub async fn create_routes(app_satte: AppState) -> Router {
    Router::new()
        .route("/api/v1/greeting", get(greeting))
        .with_state(app_satte)
}