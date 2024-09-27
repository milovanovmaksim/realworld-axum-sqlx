use super::entity::User;
use crate::app::core::error::AppError;

pub trait UsersRepository: Send + Sync + 'static {
    async fn signup(
        &self,
        email: &str,
        username: &str,
        naive_password: &str,
    ) -> Result<User, AppError>;
}
