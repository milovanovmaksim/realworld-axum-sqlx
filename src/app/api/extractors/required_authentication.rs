use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use tracing::info;
use uuid::Uuid;

use crate::app::{
    domain::jwt_token::jwt_token::JwtAuthToken,
    error::AppError,
    infrastructure::{jwt_token::jwt_token::JwtAuthTokenImpl, user::usecase::UserUseCaseImpl},
};

// Extracts the JWT from the Authorization token header.
pub struct RequiredAuthentication(pub Uuid);

impl<S> FromRequestParts<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(req, state)
                .await
                .map_err(|_| {
                    AppError::Unauthorized(String::from(
                        "Authentication is required to access this resource",
                    ))
                })?;

        info!("Attempt of getting JwtAuthTokenImpl from Extensions");
        let token_service = req
            .extensions
            .get::<Arc<JwtAuthTokenImpl>>()
            .ok_or(AppError::InternalServerError)?;

        let email = token_service.get_user_email_from_token(bearer.token())?;
        info!("User email has been found");

        info!("Attempt of getting UserRepositoryImpl from Extensions");
        let user_usecase = req
            .extensions
            .get::<Arc<UserUseCaseImpl>>()
            .ok_or(AppError::InternalServerError)?;

        match user_usecase
            .user_repository
            .get_user_by_email(email)
            .await?
        {
            Some(user) => Ok(RequiredAuthentication(user.id)),
            None => Err(AppError::Unauthorized(String::from(
                "Authentication is required to access this resource",
            ))),
        }
    }
}
