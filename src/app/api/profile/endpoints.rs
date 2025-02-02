use std::sync::Arc;

use axum::{extract, Extension, Json};
use tracing::info;

use crate::app::{
    api::{
        extractors::{
            optional_authentication::OptionalAuthentication,
            required_authentication::RequiredAuthentication,
        },
        profile::responses::ProfileResponse,
        response::ApiResponse,
    },
    domain::profile::usecase::ProfileUseCase,
    error::AppError,
    infrastructure::profile::usecase::ProfileUseCaseImpl,
};


///
/// Обработчик пути "/api/v1/profiles/{username}".
/// Возвращает информацию о профиле.
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
    security(
        ("bearer_auth" = []),
    )
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


///
/// Обработчик пути "/api/v1/profiles/{username}/follow".
/// Делает текущего пользователя подписчиком по 'username'.
/// Возвращает информацию о профиле на которого подписался текущий пользователь.
#[utoipa::path(
    post,
    path = "/api/v1/profiles/{username}/follow",
    tag = "Profile",
    description = "Follow user.", 
    responses(
        (status = StatusCode::OK, description = "User followed.", body = ProfileResponse, content_type = "application/json"),
        (status = StausCode::NOT_FOUND, description = "Profile not found.", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": "Profile with username 'DonaldTrump' not found."})),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": AppError::InternalServerError.to_string()}))
    ),
    security(
        ("bearer_auth" = []),
    )
)]
pub async fn follow_user(
    extract::Path(username): extract::Path<String>,
    Extension(profile_usecase): Extension<Arc<ProfileUseCaseImpl>>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> ApiResponse<Json<ProfileResponse>> {
    info!(
        "Recieved request to follow profile {:?} from user ID {:?}",
        username, user_id
    );
    let profile = profile_usecase.add_user_follow(user_id, username).await?;
    Ok(Json(ProfileResponse::from(profile)))
}


///
/// Обработчик пути "/api/v1/profiles/{username}/follow".
/// Отменяет подписку текущего пользователя по 'username'.
/// Возвращает информацию о профиле от которого отписался текущий пользователь.
#[utoipa::path(
    delete,
    path = "/api/v1/profiles/{username}/follow",
    tag = "Profile",
    description = "Unfollow user.", 
    responses(
        (status = StatusCode::OK, description = "User unfollowed.", body = ProfileResponse, content_type = "application/json"),
        (status = StausCode::NOT_FOUND, description = "Profile not found.", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": "Profile with username 'DonaldTrump' not found."})),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error", body = HashMap<String, String>,
            content_type = "application/json",
            example = json!({"error": AppError::InternalServerError.to_string()}))
    ),
    security(
        ("bearer_auth" = []),
    )
)]
pub async fn unfollow_user(
    extract::Path(username): extract::Path<String>,
    Extension(profile_usecase): Extension<Arc<ProfileUseCaseImpl>>,
    RequiredAuthentication(user_id): RequiredAuthentication,
) -> ApiResponse<Json<ProfileResponse>> {
    info!(
        "Recieved request to follow profile {:?} from user ID {:?}",
        username, user_id
    );
    let profile = profile_usecase
        .remove_user_follow(username, user_id)
        .await?;
    Ok(Json(ProfileResponse::from(profile)))
}
