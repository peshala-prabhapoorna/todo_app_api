use axum::{
    middleware::from_fn_with_state,
    routing::{get, patch, post, put},
    Router,
};

use crate::{
    app_state::AppState,
    middleware::require_authentication::require_authentication,
    routes::{
        greeting::greeting,
        tasks::{
            create_task::create_task, get_all_tasks::get_all_tasks, get_one_task::get_one_task, update_task::{mark_completed, mark_uncompleted, update_task},
        },
        users::{create_user::create_user, login::login, logout::logout},
    },
};

pub async fn create_routes(app_satte: AppState) -> Router {
    Router::new()
        .route("/api/v1/tasks/:task_id", patch(update_task))
        .route("/api/v1/tasks/:task_id/uncompleted", put(mark_uncompleted))
        .route("/api/v1/tasks/:task_id/completed", put(mark_completed))
        .route("/api/v1/tasks/:task_id", get(get_one_task))
        .route("/api/v1/tasks", get(get_all_tasks))
        .route("/api/v1/tasks", post(create_task))
        .route("/api/v1/users/logout", post(logout))
        .route_layer(from_fn_with_state(
            app_satte.clone(),
            require_authentication,
        ))
        .route("/api/v1/users/login", post(login))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/greeting", get(greeting))
        .with_state(app_satte)
}
