use std::sync::Arc;

use crate::{domain::like::repository::LikeRepository, features::like::error::LikeResult};

pub struct LikeService {
    repository: Arc<dyn LikeRepository>,
}

impl LikeService {
    pub fn build_from(repository: Arc<dyn LikeRepository>) -> Self {
        Self { repository }
    }

    pub async fn like_quiz(&self, user_id: i32, quiz_id: i32) -> LikeResult<()> {
        // let params = CreateQuizLikeParams { user_id, quiz_id };
        // self.like_repository.create_quiz_like(&params).await?;
        // Ok(())
        todo!()
    }

    pub async fn unlike_quiz(&self, user_id: i32, quiz_id: i32) -> LikeResult<()> {
        // let params = CreateQuizLikeParams { user_id, quiz_id };
        // self.repository.delete_quiz_like(&params).await?;
        Ok(())
    }

    pub async fn like_comment(&self, user_id: i32, comment_id: i32) -> LikeResult<()> {
        // let params = CreateCommentLikeParams {
        //     user_id,
        //     comment_id,
        // };
        // self.like_repository.create_comment_like(&params).await?;
        // Ok(())
        todo!()
    }

    pub async fn unlike_comment(&self, user_id: i32, comment_id: i32) -> LikeResult<()> {
        // let params = CreateCommentLikeParams {
        //     user_id,
        //     comment_id,
        // };
        // self.like_repository.delete_comment_like(&params).await?;
        // Ok(())
        todo!()
    }
}
