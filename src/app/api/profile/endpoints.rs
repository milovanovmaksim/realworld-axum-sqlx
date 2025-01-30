use std::{collections::HashMap, sync::Arc};

use axum::{extract::Path, Extension, Json};

use crate::app::{
    api::{self, response::ApiResponse},
    infrastructure::profile::usecase::ProfileUseCaseImpl,
};

pub async fn get_profile(
    Path(params): Path<HashMap<String, String>>,
    Extension(profile_usecase): Extension<Arc<ProfileUseCaseImpl>>,
) -> ApiResponse<Json<api::profile::responses::ProfileResponse>> {
    todo!()
}
