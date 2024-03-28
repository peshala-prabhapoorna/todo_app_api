use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ModelTrait, TryIntoModel};

use crate::utilities::app_error::AppError;

pub mod task_queries;
pub mod user_queries;

pub fn convert_active_to_model<T, M>(active_model: T) -> Result<M, AppError>
where
    T: ActiveModelTrait + TryIntoModel<M>,
    M: ModelTrait,
{
    active_model.try_into_model().map_err(|error| {
        eprintln!("Error converting active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
