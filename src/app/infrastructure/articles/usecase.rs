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
        tags::repository::TagsRepository,
    },
    error::AppError,
    infrastructure::pgsql::db::PostgreSQL,
};
use itertools::Itertools;
use slug::slugify;
use tracing::error;
use uuid::Uuid;

pub struct ArticleUseCaseImpl {
    jwt_auth_token: Arc<dyn JwtAuthToken>,
    articles_repository: Arc<dyn ArticlesRepository>,
    tags_repository: Arc<dyn TagsRepository>,
    pg_sql: PostgreSQL,
}

impl ArticleUseCaseImpl {
    pub fn new(
        jwt_auth_token: Arc<dyn JwtAuthToken>,
        articles_repository: Arc<dyn ArticlesRepository>,
        tags_repository: Arc<dyn TagsRepository>,
        pg_sql: PostgreSQL,
    ) -> Self {
        Self {
            jwt_auth_token,
            articles_repository,
            tags_repository,
            pg_sql,
        }
    }
}

impl ArticlesUsecase for ArticleUseCaseImpl {
    async fn cereate_article(
        &self,
        request: CreateArticleUsecaseRequest,
    ) -> Result<ArticleUsecaseResponse, AppError> {
        let uuid = Uuid::new_v4().to_string();
        let slug = slugify(format!("{}-{}", request.title, uuid));

        let deduped_tag_list = request.tag_list.clone().into_iter().unique().collect_vec();
        let existing_tags = self
            .tags_repository
            .get_tags(deduped_tag_list.clone())
            .await?
            .into_iter()
            .map(|tag| tag.tag)
            .collect_vec();

        let mut tags_to_create: Vec<String> = Vec::new();

        for tag in deduped_tag_list.clone() {
            if !existing_tags.contains(&tag) {
                tags_to_create.push(tag);
            }
        }

        let mut tx = self.pg_sql.begin().await?;

        let created_article = self
            .articles_repository
            .create_article(
                Some(&mut *tx),
                CreateArticleRepoRequest::from((request, slug.clone())),
            )
            .await;
        if let Err(error) = created_article {
            error!("Failed to create new article {:?}.", error);
            tx.rollback().await?;
            return Err(error);
        }

        if !tags_to_create.is_empty() {
            let created_tags = self.tags_repository.create_tags(tags_to_create).await;
            if let Err(error) = created_tags {
                error!("Faled to create new tags {:?}", error);
                tx.rollback().await?;
                return Err(error);
            }
        }

        let created_article = created_article.unwrap();

        let article_tags_to_create = self
            .tags_repository
            .get_tags(deduped_tag_list.clone())
            .await?
            .into_iter()
            .map(|tag| (tag.id, created_article.id))
            .collect_vec();

        tx.commit().await?;

        todo!()
    }
}
