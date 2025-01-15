use axum::{routing::get, Json, Router};
use utoipa::OpenApi;

use crate::app::api::user::{requests::SignupUserRequest, responses::SignupUserResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::app::api::user::router::signup,
        openapi
    ),
    components(
        schemas(SignupUserRequest, SignupUserResponse),
    ),
    tags(
        (name = "users", description = "Users endpoints"),
        (name = "api-docs", description = "Openapi endpoints")
    ),
)]
pub struct ApiDocumentation;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
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
