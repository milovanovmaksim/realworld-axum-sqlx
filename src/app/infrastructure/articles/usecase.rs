use std::sync::Arc;

use crate::app::{
    domain::{
        articles::{
            repository::{requests::CreateArticleRepoRequest, ArticlesRepository},
            usecase::{
                requests::CreateArticleUsecaseRequest, responses::ArticleUsecaseResponse,
                ArticlesUsecase,
            },
        },
        jwt_token::jwt_token::JwtAuthToken,
    },
    error::AppError,
    infrastructure::pgsql::db::PostgreSQL,
};
use slug::slugify;
use tracing::error;

pub struct ArticleUseCaseImpl {
    jwt_auth_token: Arc<dyn JwtAuthToken>,
    articles_repository: Arc<dyn ArticlesRepository>,
    pg_sql: PostgreSQL,
}

impl ArticleUseCaseImpl {
    pub fn new(
        jwt_auth_token: Arc<dyn JwtAuthToken>,
        articles_repository: Arc<dyn ArticlesRepository>,
        pg_sql: PostgreSQL,
    ) -> Self {
        Self {
            jwt_auth_token,
            articles_repository,
            pg_sql,
        }
    }
}

impl ArticlesUsecase for ArticleUseCaseImpl {
    async fn cereate_article(
        &self,
        request: CreateArticleUsecaseRequest,
    ) -> Result<ArticleUsecaseResponse, AppError> {
        let slug = slugify(&request.title);

        let mut tx = self.pg_sql.begin().await?;

        let article = self
            .articles_repository
            .create_article(
                Some(&mut *tx),
                CreateArticleRepoRequest::from((request, slug)),
            )
            .await;
        if let Err(error) = article {
            error!("Failed to create new article {:?}.", error);
            tx.rollback().await?;
            return Err(error);
        }

        tx.commit().await?;

        todo!()
    }
}
