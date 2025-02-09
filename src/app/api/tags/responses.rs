use serde::Serialize;
use utoipa::ToSchema;

///
/// Ответ api сервера. Содержит список тэгов.
#[derive(Debug, Serialize, ToSchema)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

impl From<Vec<String>> for TagsResponse {
    fn from(value: Vec<String>) -> Self {
        TagsResponse { tags: value }
    }
}
