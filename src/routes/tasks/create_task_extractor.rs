use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    Json, RequestExt,
};
use serde::Deserialize;
use validator::Validate;

use crate::utilities::app_error::AppError;

#[derive(Debug, Deserialize, Validate)]
pub struct ValidateCreateTask {
    #[validate(length(equal = 1, message = "Priority must be a single character"))]
    pub priority: Option<String>,
    #[validate(required(message = "missing task title"))]
    pub title: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
impl<S> FromRequest<S> for ValidateCreateTask
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(request: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(task) = request
            .extract::<Json<ValidateCreateTask>, _>()
            .await
            .map_err(|error| {
                eprintln!("Error extracting new task {:?}", error);
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong please try again",
                )
            })?;

        if let Err(error) = task.validate() {
            let field_errors = error.field_errors();
            for (_, error) in field_errors {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    // feel safe unwrapping because we know that there is at least one error,
                    // and we only care about the first for this api
                    error.first().unwrap().clone().message.unwrap().to_string(),
                ));
            }
        }

        Ok(task)
    }
}
