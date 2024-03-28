use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::tasks::{self, Entity as Tasks, Model as TaskModel},
    utilities::app_error::AppError,
};

use super::convert_active_to_model;

pub async fn save_active_task(
    db: &DatabaseConnection,
    task: tasks::ActiveModel,
) -> Result<TaskModel, AppError> {
    let task = task.save(db).await.map_err(|error| {
        eprintln!("Error saving task: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving task")
    })?;

    convert_active_to_model(task)
}

pub async fn get_default_tasks(db: &DatabaseConnection) -> Result<Vec<TaskModel>, AppError> {
    Tasks::find()
        .filter(tasks::Column::IsDefault.eq(Some(true)))
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting default tasks: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting default tasks",
            )
        })
}
