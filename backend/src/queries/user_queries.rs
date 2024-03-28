use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{
    database::users::{self, Model as UserModel},
    utilities::app_error::AppError,
};

use super::convert_active_to_model;

pub async fn save_active_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UserModel, AppError> {
    let user = user.save(db).await.map_err(|error| {
        let error_message = &error.to_string();
        if error_message
            .contains("uplicate key value violates unique constraint \"users_username_key\"")
        {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different user name",
            )
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again.",
            )
        }
    })?;

    convert_active_to_model(user)
}
