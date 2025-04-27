pub mod requests;
pub mod responses;

use async_trait::async_trait;
use requests::{SigninUserUsecaseRequest, SignupUserUsecaseRequest, UpdateUserUsecaseRequest};
use responses::UserUsecase;
use uuid::Uuid;

use crate::app::error::AppError;

///
/// Интерфейс, определяющий набор методов бизнес логики пользователя.
#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    ///
    /// Регистрирует нового пользователя.
    async fn signup(&self, request: SignupUserUsecaseRequest) -> Result<UserUsecase, AppError>;

    ///
    /// Авторизация пользователя.
    async fn login(&self, request: SigninUserUsecaseRequest) -> Result<UserUsecase, AppError>;
    async fn get_current_user(&self, user_id: Uuid) -> Result<UserUsecase, AppError>;

    ///
    /// Обновляет информацию о пользователе.
    async fn update_user(
        &self,
        request: (Uuid, UpdateUserUsecaseRequest),
    ) -> Result<UserUsecase, AppError>;
}
