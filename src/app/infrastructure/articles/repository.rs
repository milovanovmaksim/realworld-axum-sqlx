use async_trait::async_trait;

use crate::app::{
    domain::articles::repository::{
        requests::CreateArticleRequest, responses::ArticleEntity, ArticlesRepository,
    },
    error::AppError,
    infrastructure::pgsql::db::PostgreSQL,
};

pub struct ArticlesRepositoryImpl {
    pg_sql: PostgreSQL,
}

impl ArticlesRepositoryImpl {
    pub fn new(pg_sql: PostgreSQL) -> Self {
        Self { pg_sql }
    }
}

#[async_trait]
impl ArticlesRepository for ArticlesRepositoryImpl {
    async fn create_article(
        &self,
        article: CreateArticleRequest,
    ) -> Result<ArticleEntity, AppError> {
        todo!()
    }
}
