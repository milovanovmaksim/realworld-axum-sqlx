pub mod responses;

use responses::ProfileResponse;

use crate::app::domain::{error::AppError, user::repository::Email};

trait ProfileUseCase {
    async fn get_profile(
        &self,
        current_user: Email,
        username: &str,
    ) -> Result<ProfileResponse, AppError>;
}
