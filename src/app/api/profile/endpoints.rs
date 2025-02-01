use std::sync::Arc;

use axum::{extract, Extension, Json};
use tracing::info;

use crate::app::{
    api::{
        extractors::optional_authentication::OptionalAuthentication,
        profile::responses::ProfileResponse, response::ApiResponse,
    },
    domain::profile::usecase::ProfileUseCase,
    error::AppError,
    infrastructure::profile::usecase::ProfileUseCaseImpl,
};

#[utoipa::path(
    get,
    path = "/api/v1/profiles/{username}",
    tag = "Profile",
    description = "Get profile.", 
    responses(
            (status = StatusCode::OK, description = "Profile", body = ProfileResponse, content_type = "application/json"),
            (status = StausCode::NOT_FOUND, description = "Profile not found.", body = HashMap<String, String>,
                content_type = "application/json",
                example = json!({"error": "Profile with username 'DonaldTrump' not found."})),
            (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = HashMap<String, String>,
                content_type = "application/json",
                example = json!({"error": AppError::InternalServerError.to_string()}))
            ),
)]
pub async fn get_profile(
    extract::Path(username): extract::Path<String>,
    Extension(profile_usecase): Extension<Arc<ProfileUseCaseImpl>>,
    OptionalAuthentication(user_id): OptionalAuthentication,
) -> ApiResponse<Json<ProfileResponse>> {
    info!("Recieved request to get profile {:?}", username);
    let profile = profile_usecase.get_profile(user_id, username).await?;
    Ok(Json(ProfileResponse::from(profile)))
}
