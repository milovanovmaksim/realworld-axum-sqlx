use std::{path::Path, sync::Arc};

use super::{
    jwt_token::{jwt_token::JwtAuthTokenImpl, settings::JwtTokenSettings},
    pgsql::{db::PostgreSQL, settings::DatabaseSettings, transaction::Transaction},
    profile::{repository::ProfileRepositoryImpl, usecase::ProfileUseCaseImpl},
    tags::{repository::TagsRepositoryImpl, usecase::TagsUsacaseImpl},
    user::{repository::UsersRepositoryImpl, usecase::UserUseCaseImpl},
};

///
/// Контейнер внедрения зависимостей.
pub struct DiContainer {
    // User
    pub user_repository: Arc<UsersRepositoryImpl>,
    pub user_usecase: Arc<UserUseCaseImpl>,

    // Profile
    pub profile_repository: Arc<ProfileRepositoryImpl>,
    pub profile_usecase: Arc<ProfileUseCaseImpl>,

    // Tags
    pub tags_repository: Arc<TagsRepositoryImpl>,
    pub tags_usecase: Arc<TagsUsacaseImpl>,

    // Utility services
    pub jwt_auth_token: Arc<JwtAuthTokenImpl>,
}

impl DiContainer {
    ///
    /// Основной конструктор.
    pub async fn new<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        // Utility services
        let jwt_auth_token = Arc::new(JwtAuthTokenImpl::new(
            JwtTokenSettings::from_yaml(path.as_ref()).map_err(|e| {
                format!("DiContainer::new || error: failed to create jwt auth token service {e}")
            })?,
        ));
        let pg_sql =
            PostgreSQL::configure_database(DatabaseSettings::from_yaml(path.as_ref()).map_err(
                |e| format!("DiContainer::new || error: failed to create postgresql client {e}"),
            )?)
            .await;

        // User
        let user_repository = Arc::new(UsersRepositoryImpl::new(pg_sql.clone()));
        let user_usecase = Arc::new(UserUseCaseImpl::new(
            user_repository.clone(),
            jwt_auth_token.clone(),
        ));

        // Profile
        let profile_repository = Arc::new(ProfileRepositoryImpl::new(pg_sql.clone()));
        let profile_usecase = Arc::new(ProfileUseCaseImpl::new(
            user_repository.clone(),
            profile_repository.clone(),
        ));

        // Tags
        let tags_repository = Arc::new(TagsRepositoryImpl::new(pg_sql.clone()));
        let tags_usecase = Arc::new(TagsUsacaseImpl::new(tags_repository.clone()));

        Ok(Self {
            user_repository,
            user_usecase,
            profile_repository,
            profile_usecase,
            tags_repository,
            tags_usecase,
            jwt_auth_token,
        })
    }
}
