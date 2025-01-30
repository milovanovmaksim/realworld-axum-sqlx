use std::sync::Arc;

use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use tracing::{info, warn};
use uuid::Uuid;

use crate::app::{
    domain::jwt_token::jwt_token::JwtAuthToken,
    error::AppError,
    infrastructure::{jwt_token::jwt_token::JwtAuthTokenImpl, user::usecase::UserUseCaseImpl},
};

///
/// Extracts the JWT from the Authorization token header, optional and will not return errors if none is found.
pub struct OptionalAuthentication(pub Option<Uuid>);

impl<S> FromRequestParts<S> for OptionalAuthentication
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let optional_token_response = Ok(OptionalAuthentication(None));

        match TypedHeader::<Authorization<Bearer>>::from_request_parts(req, state).await {
            Ok(TypedHeader(Authorization(bearer))) => {
                info!("Attempt of getting JwtAuthTokenImpl from Extensions");
                let token_service = req
                    .extensions
                    .get::<Arc<JwtAuthTokenImpl>>()
                    .ok_or(AppError::InternalServerError)?;

                match token_service.get_user_email_from_token(bearer.token()) {
                    Ok(email) => {
                        info!("Attempt of getting UserRepositoryImpl from Extensions");
                        let user_usecase = req
                            .extensions
                            .get::<Arc<UserUseCaseImpl>>()
                            .ok_or(AppError::InternalServerError)?;
                        match user_usecase
                            .user_repository
                            .get_user_by_email(email.clone())
                            .await?
                        {
                            Some(user) => Ok(OptionalAuthentication(Some(user.id))),
                            None => {
                                warn!("User with email '{}' not found.", email);
                                optional_token_response
                            }
                        }
                    }
                    Err(_) => {
                        warn!("Request does not contain a valid token");
                        optional_token_response
                    }
                }
            }
            Err(_) => {
                warn!("Request does not contain a token.");
                optional_token_response
            }
        }
    }
}
