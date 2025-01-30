use std::{collections::HashMap, sync::Arc};

use axum::{extract::Path, Extension, Json};
use tracing::info;

use crate::app::{
    api::{
        self, extractors::optional_authentication::OptionalAuthentication, response::ApiResponse,
    },
    domain::profile::usecase::ProfileUseCase,
    infrastructure::profile::usecase::ProfileUseCaseImpl,
};

pub async fn get_profile(
    Path(params): Path<HashMap<String, String>>,
    Extension(profile_usecase): Extension<Arc<ProfileUseCaseImpl>>,
    OptionalAuthentication(user_id): OptionalAuthentication,
) -> ApiResponse<Json<api::profile::responses::ProfileResponse>> {
    let username = params.get("username").unwrap();
    info!("recieved request to get profile {:?}", username);
    let profile = profile_usecase.get_profile(user_id, username).await?;
    todo!()
}
