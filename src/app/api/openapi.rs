use axum::{routing::get, Json, Router};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::app::api::{
    profile::responses::ProfileResponse,
    user::{
        requests::{SigninUserRequest, SignupUserRequest, UpdateUserRequest},
        responses::AuthenticationUserResponse,
    },
};

#[derive(OpenApi)]
#[openapi(
    paths(
        super::user::endpoints::signup,
        super::user::endpoints::login,
        super::user::endpoints::get_current_user,
        super::user::endpoints::update_user,
        super::profile::endpoints::get_profile,
        super::profile::endpoints::follow_user,
        openapi,
    ),
    components(
        schemas(SignupUserRequest, AuthenticationUserResponse, SigninUserRequest, UpdateUserRequest, ProfileResponse),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "User and Authentication", description = "User endpoints"),
        (name = "Profile", description = "Profile endpoints"),
        (name = "api-docs", description = "Openapi endpoints")
    ),
)]
pub struct ApiDocumentation;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

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
