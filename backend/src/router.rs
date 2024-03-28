use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    routes::{greeting::greeting, users::create_user::create_user},
};

pub async fn create_routes(app_satte: AppState) -> Router {
    Router::new()
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/greeting", get(greeting))
        .with_state(app_satte)
}
