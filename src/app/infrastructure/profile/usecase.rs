use std::sync::Arc;

use async_trait::async_trait;

use crate::app::{
    domain::{
        self,
        profile::{repository::ProfileRepository, usecase::ProfileUseCase},
        user::repository::{Email, UserRepository},
    },
    error::AppError,
};

pub struct ProfileUseCaseImpl {
    pub user_repository: Arc<dyn UserRepository>,
    pub profile_repository: Arc<dyn ProfileRepository>,
}

#[async_trait]
impl ProfileUseCase for ProfileUseCaseImpl {
    async fn get_profile(
        &self,
        current_user: Email,
        username: String,
    ) -> Result<domain::profile::usecase::responses::ProfileResponse, AppError> {
        todo!()
    }
}
