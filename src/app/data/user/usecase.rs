use std::sync::Arc;

use crate::app::domain::{
    error::AppError,
    user::{
        repository::UserRepository,
        usecase::{SignupUserResult, UserUseCase},
    },
};

#[derive(Clone)]
pub struct UserUseCaseImpl {
    pub user_repository: Arc<dyn UserRepository>,
}

impl UserUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

impl UserUseCase for UserUseCaseImpl {
    async fn signup(
        &self,
        username: &str,
        email: &str,
        naive_password: &str,
    ) -> Result<SignupUserResult, AppError> {
        let user = self
            .user_repository
            .signup(email, username, naive_password)
            .await?;
        let token = user.generate_token();
        Ok(SignupUserResult {
            id: user.id,
            username: user.username,
            email: user.email,
            password: user.password,
            bio: user.bio,
            image: user.image,
            created_at: user.created_at,
            updated_at: user.updated_at,
            token: token,
        })
    }
}
