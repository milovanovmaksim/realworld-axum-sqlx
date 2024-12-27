use async_trait::async_trait;
use axum::{extract::FromRequest, Json};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::app::domain::error::AppError;

pub struct ValidationExtractor<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidationExtractor<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidationExtractor(value))
    }
}
