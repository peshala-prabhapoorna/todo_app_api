use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::database::users::{self, Entity as Users};
use crate::utilities::{app_error::AppError, jwt::validate_token, token_wrapper::TokenWrapper};

pub async fn require_authentication(
    headers: HeaderMap,
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let header_token = if let Some(token) = headers.get("x-auth-token") {
        token.to_str().map_err(|error| {
            eprintln!("Error extracting data from headers: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
        })?
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "not authenticated!",
        ));
    };

    validate_token(&*token_secret, &header_token)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(header_token.to_string())))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by token: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a problem getting your account.",
            )
        })?;

    if let Some(user) = user {
        request.extensions_mut().insert(user);
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized for this",
        ));
    }

    Ok(next.run(request).await)
}
