use std::sync::Arc;

use axum::{Extension, Json};
use tracing::info;

use crate::app::{
    api::response::ApiResponse, domain::tags::usecase::TagsUseCase, error::AppError,
    infrastructure::tags::usecase::TagsUsacaseImpl,
};

use super::responses::TagsResponse;

#[utoipa::path(get,
    path = "/api/v1/tags",
    tag = "Tags",
    description = "Get tags",
    responses(
        (status = StatusCode::OK, description = "Get tags.", body = TagsResponse, content_type = "application/json"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": AppError::InternalServerError.to_string()}))
    ),
)]
pub async fn get_tags(
    Extension(tags_usecase): Extension<Arc<TagsUsacaseImpl>>,
) -> ApiResponse<Json<TagsResponse>> {
    info!("recieved request to retrieve all tags");

    let tags = tags_usecase.get_tags().await?;
    Ok(Json(TagsResponse::from(tags)))
}
