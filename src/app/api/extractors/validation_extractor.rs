use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::app::domain::error::AppError;

pub struct ValidationExtractor<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidationExtractor<T>
where
    T: DeserializeOwned + Validate,
    B: Send,
{
    type Rejection = AppError;

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        todo!()
    }
}
