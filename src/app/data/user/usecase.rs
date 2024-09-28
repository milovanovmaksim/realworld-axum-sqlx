use std::sync::Arc;

use crate::app::domain::{error::AppError, user::{repository::UserRepository, usecase::{SignupUserResult, UserUseCase}}};


#[derive(Clone)]
pub struct UserUseCaseImpl {
    pub user_repository: Arc<dyn UserRepository>,
}


impl UserUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self {
            user_repository
        }
        
    }
}

impl UserUseCase for UserUseCaseImpl {
    async fn signup(
            &self,
            username: &str,
            email: &str,
            naive_password: &str,
        ) -> Result<SignupUserResult, AppError> {
            todo!()
        
    }
}