use std::sync::Arc;
use crate::{
    domain::repositories::comment_repository::CommentRepository,
    domain::repositories::user_repository::UserRepository,
    domain::models::comment::Comment,
    application::error::{ServiceError, ServiceResult},
    interface::dto::comment_dto::{CommentDto, CreateCommentDto, ListCommentsQuery},
};

#[derive(Clone, Default)] // Default for placeholder in AppState
pub struct CommentService {
    // comment_repo: Arc<dyn CommentRepository>,
    // user_repo: Arc<dyn UserRepository>,
}

impl CommentService {
    pub fn new(/*...dependencies...*/) -> Self {
        Self { /*...dependencies...*/ }
    }

    pub async fn list_comments(&self, query: ListCommentsQuery) -> ServiceResult<(Vec<CommentDto>, i64, i64)> {
        // let comments = self.comment_repo.list(&query).await?;
        // let total_items = self.comment_repo.count(&query).await?; // Need a count method in repo
        // let total_pages = (total_items as f64 / query.limit.unwrap_or(10) as f64).ceil() as i64;

        // let comment_dtos: Vec<CommentDto> = comments.into_iter().map(|c| {
        //     // Fetch user display name for each comment
        //     let user_display_name = self.user_repo.find_by_id(c.cmt_user_id.unwrap_or_default()).await?
        //         .map(|u| u.usr_display_name).unwrap_or_else(|| "Unknown User".to_string());
        //     CommentDto::from_domain(c, user_display_name)
        // }).collect();

        println!("Listing comments for quiz {}: {:?}", query.quiz_id, query); // Placeholder
        Ok((vec![], 0, 0))
    }

    pub async fn create_comment(&self, user_id: i32, dto: CreateCommentDto) -> ServiceResult<CommentDto> {
        // let comment = self.comment_repo.create(user_id, &dto).await?;
        // let user = self.user_repo.find_by_id(user_id).await?
        //     .ok_or_else(|| ServiceError::NotFound(format!("User with ID {} not found", user_id)))?;
        println!("Creating comment by user {} for quiz {}", user_id, dto.quiz_id); // Placeholder
        Err(ServiceError::Internal)
    }

    pub async fn delete_comment(&self, comment_id: i32, user_id: i32) -> ServiceResult<()> {
        // let comment = self.comment_repo.find_by_id(comment_id).await?
        //     .ok_or_else(|| ServiceError::NotFound(format!("Comment with ID {} not found", comment_id)))?;

        // // Authorization check
        // if comment.cmt_user_id != Some(user_id) {
        //     // Also check if user is admin
        //     return Err(ServiceError::Unauthorized("You are not authorized to delete this comment".to_string()));
        // }

        // self.comment_repo.delete(comment_id).await?;
        println!("Deleting comment ID: {} by user {}", comment_id, user_id); // Placeholder
        Ok(())
    }
}
