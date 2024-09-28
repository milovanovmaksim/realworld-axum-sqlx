use super::entity::User;
use crate::app::domain::error::AppError;

pub trait UserRepository: Send + Sync {
    async fn signup(
        &self,
        email: &str,
        username: &str,
        naive_password: &str,
    ) -> Result<User, AppError>;
}
