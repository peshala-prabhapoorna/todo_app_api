use axum::{extract::{Path, State}, Extension};
use chrono::Utc;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use crate::{database::users::Model as UserModel, queries::task_queries::{find_task_by_id, save_active_task}, utilities::app_error::AppError};

pub async fn soft_delete_task(
    Path(task_id): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut task = find_task_by_id(&db, task_id, user.id)
        .await?
        .into_active_model();

    let now = Utc::now();
    task.deleted_at = Set(Some(now.into()));
    save_active_task(&db, task).await?;

    Ok(())
}