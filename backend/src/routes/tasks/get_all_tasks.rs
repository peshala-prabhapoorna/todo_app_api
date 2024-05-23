use axum::{extract::State, Extension, Json};
use sea_orm::DatabaseConnection;

use crate::{
    database::users::Model as UserModel, queries::task_queries, routes::tasks::ResponseTask,
    utilities::app_error::AppError,
};

use super::ResponseDataTasks;

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let tasks = task_queries::get_all_tasks(&db, user.id, false)
        .await?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            priority: db_task.priority,
            title: db_task.title,
            description: db_task.description,
            completed_at: db_task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(ResponseDataTasks { data: tasks }))
}
