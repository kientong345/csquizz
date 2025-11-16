use std::sync::Arc;

use crate::{
    domain::{
        comment::{model::Comment, repository::ICommentRepository},
        page::Page,
    },
    features::comment::{dto::CommentDetailDto, error::CommentResult},
};

pub struct CommentService {
    repository: Arc<dyn ICommentRepository>,
}

impl CommentService {
    pub fn build_from(repository: Arc<dyn ICommentRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_comment(
        &self,
        quiz_id: i32,
        user_id: i32,
        content: String,
    ) -> CommentResult<CommentDetailDto> {
        // let params = CreateCommentParams {
        //     quiz_id,
        //     user_id,
        //     content,
        // };
        // let comment = self.comment_repository.create(&params).await?;

        // // After creating, we might want to return the full CommentWithDetails,
        // // but for now, we'll just refetch it to get all details.
        // // A more optimized way would be to have the create method return CommentWithDetails.
        // let (details, _) = self
        //     .comment_repository
        //     .list(&ListCommentsParams {
        //         quiz_id: comment.quiz_id,
        //         page: 1,
        //         limit: 1, // Assuming we can filter by comment id later, for now this is a workaround
        //     })
        //     .await?;

        // let detail = details
        //     .into_iter()
        //     .find(|d| d.comment.id == comment.id)
        //     .ok_or(RepositoryError::NotFound)?;

        // Ok(CommentDto {
        //     id: detail.comment.id,
        //     content: detail.comment.content,
        //     created_at: detail.comment.created_at.to_rfc3339(),
        //     user_id: detail.author.usr_id,
        //     user_display_name: detail.author.usr_display_name,
        //     user_avatar_url: detail.author.usr_avatar_url,
        //     like_count: detail.like_count,
        //     is_liked_by_user: None, // Requires passing current user context
        // })
        todo!()
    }

    pub async fn list_comments(
        &self,
        quiz_id: i32,
        page: u32,
        limit: u32,
    ) -> CommentResult<Page<CommentDetailDto>> {
        // let params = ListCommentsParams {
        //     quiz_id,
        //     page,
        //     limit,
        // };
        // let (comments_with_details, total_items) = self.comment_repository.list(&params).await?;

        // let comment_dtos = comments_with_details
        //     .into_iter()
        //     .map(|detail| CommentDto {
        //         id: detail.comment.id,
        //         content: detail.comment.content,
        //         created_at: detail.comment.created_at.to_rfc3339(),
        //         user_id: detail.author.usr_id,
        //         user_display_name: detail.author.usr_display_name,
        //         user_avatar_url: detail.author.usr_avatar_url,
        //         like_count: detail.like_count,
        //         is_liked_by_user: None, // Requires passing current user context
        //     })
        //     .collect();

        // let pagination_info = PaginationInfo {
        //     current_page: page,
        //     total_pages: (total_items as f64 / limit as f64).ceil() as u32,
        //     total_items,
        //     limit,
        // };

        // Ok(PaginatedCommentsDto {
        //     pagination: pagination_info,
        //     data: comment_dtos,
        // })
        todo!()
    }

    pub async fn delete_comment(&self, comment_id: i32, current_user_id: i32) -> CommentResult<()> {
        // let comment = self
        //     .comment_repository
        //     .find_by_id(comment_id)
        //     .await?
        //     .ok_or(RepositoryError::NotFound)?;

        // // Basic permission check
        // if comment.user_id != current_user_id {
        //     // In a real app, you might also allow quiz owner or admin to delete
        //     return Err(crate::application::error::ServiceError::Forbidden);
        // }

        // self.comment_repository.delete(comment_id).await?;
        // Ok(())
        todo!()
    }
}
