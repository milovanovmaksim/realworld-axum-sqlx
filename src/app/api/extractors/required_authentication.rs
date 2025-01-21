use axum::{extract::FromRequest, http::header::AUTHORIZATION, Extension};
use uuid::Uuid;

use crate::app::{domain::{error::AppError, jwt_token::jwt_token::JwtAuthToken}, infrastructure::jwt_token::jwt_token::JwtAuthTokenImpl};


// Extracts the JWT from the Authorization token header.
pub struct RequiredAuthentication(pub Uuid);

impl<S> FromRequest<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: axum::extract::Request, state: &S,) -> Result<Self, Self::Rejection> {
        if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
            let header_value = auth_header.to_str().map_err(|_| 
                AppError::Unauthorized(String::from("Authentication is required to access this resource")))?;

            if !header_value.contains("Token") {
                tracing::error!("request does not contain valid 'Token' prefix for authorization");
                return Err(AppError::Unauthorized(String::from("Authentication is required to access this resource")));
            }

            let tokenized_value: Vec<&str> = header_value.split(' ').collect();

            if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                tracing::error!("request does not contain valid token");
                return Err(AppError::Unauthorized(String::from("Authentication is required to access this resource")));
            }

            let token_value = tokenized_value.into_iter().nth(1).unwrap().to_string();

            let Extension(token_service): Extension<JwtAuthTokenImpl> = Extension::from_request(req, state).await
                .map_err(|_| AppError::InternalServerError)?;

            let user_id = token_service.get_user_id_from_token(token_value)?;
            
            Ok(RequiredAuthentication(user_id))
        } else {
            return Err(AppError::Unauthorized(String::from("Authentication is required to access this resource")));
        }


        
    }
}
