use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(crate::app::api::user::router::signup))]
pub struct ApiDocunentation;
