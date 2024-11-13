use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::DatabaseConnection;

use crate::{
    database::users::Model as UserModel, queries::task_queries, routes::tasks::ResponseTask,
    utilities::app_error::AppError,
};

use super::{create_task_extractor::ValidateCreateTask, ResponseDataTask};

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let task = task_queries::create_task(task, &user, &db).await?;

    let response = ResponseTask {
        id: task.id,
        priority: task.priority,
        title: task.title,
        description: task.description,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };

    Ok((
        StatusCode::CREATED,
        Json(ResponseDataTask { data: response }),
    ))
}
