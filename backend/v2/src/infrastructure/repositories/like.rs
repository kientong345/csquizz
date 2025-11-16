use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{error::RepositoryResult, like::repository::ILikeRepository},
    infrastructure::database::postgres_context::DatabasePool,
};

pub struct LikeRepository {
    pool: Arc<DatabasePool>,
}

impl LikeRepository {
    pub fn init(pool: Arc<DatabasePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ILikeRepository for LikeRepository {
    async fn create_quiz_like(&self, user_id: i32, quiz_id: i32) -> RepositoryResult<()> {
        // sqlx::query!(
        //     // "ON CONFLICT DO NOTHING" handles cases where the like already exists,
        //     // preventing duplicate entries and errors.
        //     "INSERT INTO quiz_likes (qzlk_user_id, qzlk_quiz_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        //     params.user_id,
        //     params.quiz_id
        // )
        // .execute(&self.pool)
        // .await?;
        // Ok(())
        todo!()
    }

    async fn delete_quiz_like(&self, quiz_like_id: i32) -> RepositoryResult<()> {
        // sqlx::query!(
        //     "DELETE FROM quiz_likes WHERE qzlk_user_id = $1 AND qzlk_quiz_id = $2",
        //     params.user_id,
        //     params.quiz_id
        // )
        // .execute(&self.pool)
        // .await?;
        // Ok(())
        todo!()
    }

    async fn create_comment_like(&self, user_id: i32, comment_id: i32) -> RepositoryResult<()> {
        // sqlx::query!(
        //     "INSERT INTO comment_likes (cmtlk_user_id, cmtlk_comment_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        //     params.user_id,
        //     params.comment_id
        // )
        // .execute(&self.pool)
        // .await?;
        // Ok(())
        todo!()
    }

    async fn delete_comment_like(&self, comment_like_id: i32) -> RepositoryResult<()> {
        // sqlx::query!(
        //     "DELETE FROM comment_likes WHERE cmtlk_user_id = $1 AND cmtlk_comment_id = $2",
        //     params.user_id,
        //     params.comment_id
        // )
        // .execute(&self.pool)
        // .await?;
        // Ok(())
        todo!()
    }
}
