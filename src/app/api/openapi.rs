use axum::{routing::get, Json, Router};
use utoipa::OpenApi;

use crate::app::api::user::{requests::{SignupUserRequest, SigninUserRequest}, responses::AuthenticationUserResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        super::user::endpoints::signup,
        super::user::endpoints::login,
        openapi
    ),
    components(
        schemas(SignupUserRequest, AuthenticationUserResponse, SigninUserRequest),
    ),
    tags(
        (name = "User and Authentication", description = "Users endpoints"),
        (name = "api-docs", description = "Openapi endpoints")
    ),
)]
pub struct ApiDocumentation;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    tag = "api-docs",
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDocumentation::openapi())
}

pub fn router() -> Router {
    axum::Router::new().route("/api-docs/openapi.json", get(openapi))
}
