use std::sync::Arc;

use crate::domain::quiz::repository::IQuizRepository;

pub struct QuizService {
    repository: Arc<dyn IQuizRepository>,
}

impl QuizService {
    pub fn build_from(repository: Arc<dyn IQuizRepository>) -> Self {
        Self { repository }
    }

    // pub async fn create_quiz(
    //     &self,
    //     title: String,
    //     description: Option<String>,
    //     difficulty: &str,
    //     category_id: i32,
    //     creator_id: i32,
    // ) -> ServiceResult<QuizDetailDto> {
    //     let difficulty_enum = QuizDifficulty::from_string(difficulty)
    //         .ok_or_else(|| ServiceError::bad_request("Invalid difficulty level"))?;

    //     let params = CreateQuizParams {
    //         title,
    //         description,
    //         difficulty: difficulty_enum,
    //         category_id,
    //         creator_id,
    //     };

    //     let created_quiz = self.quiz_repository.create(&params).await?;

    //     self.get_quiz_detail(created_quiz.id).await
    // }

    // pub async fn get_quiz_detail(&self, quiz_id: i32) -> ServiceResult<QuizDetailDto> {
    //     let quiz_detail = self
    //         .quiz_repository
    //         .find_detail_by_id(quiz_id)
    //         .await?
    //         .ok_or(RepositoryError::NotFound)?;

    //     Ok(QuizDetailDto {
    //         id: quiz_detail.quiz.id,
    //         title: quiz_detail.quiz.title,
    //         description: quiz_detail.quiz.description,
    //         difficulty: quiz_detail.quiz.difficulty.to_string(),
    //         question_count: quiz_detail.question_count,
    //         like_count: quiz_detail.like_count,
    //         comment_count: quiz_detail.comment_count,
    //         category_id: quiz_detail.category.id,
    //         category_name: quiz_detail.category.name,
    //         creator_id: quiz_detail.creator.usr_id,
    //         creator_display_name: quiz_detail.creator.usr_display_name,
    //         creator_avatar_url: quiz_detail.creator.usr_avatar_url,
    //         created_at: quiz_detail.quiz.created_at.to_rfc3339(),
    //         updated_at: quiz_detail.quiz.updated_at.to_rfc3339(),
    //     })
    // }

    // pub async fn list_quizzes(
    //     &self,
    //     category_id: Option<i32>,
    //     difficulty: Option<String>,
    //     search_term: Option<String>,
    //     page: u32,
    //     limit: u32,
    //     sort_by: Option<String>,
    //     order: Option<String>,
    // ) -> ServiceResult<PaginatedQuizzesDto> {
    //     let difficulty_enum = difficulty
    //         .map(|d| {
    //             QuizDifficulty::from_string(&d)
    //                 .ok_or_else(|| ServiceError::bad_request("Invalid difficulty level"))
    //         })
    //         .transpose()?;

    //     let sort_by_enum = sort_by
    //         .map(|s| match s.as_str() {
    //             "created_at" => Ok(QuizSortField::CreatedAt),
    //             "like_count" => Ok(QuizSortField::LikeCount),
    //             "comment_count" => Ok(QuizSortField::CommentCount),
    //             "question_count" => Ok(QuizSortField::QuestionCount),
    //             "title" => Ok(QuizSortField::Title),
    //             _ => Err(ServiceError::bad_request("Invalid sort_by field")),
    //         })
    //         .transpose()?;

    //     let order_enum = order
    //         .map(|o| match o.as_str() {
    //             "asc" => Ok(SortOrder::Asc),
    //             "desc" => Ok(SortOrder::Desc),
    //             _ => Err(ServiceError::bad_request("Invalid order field")),
    //         })
    //         .transpose()?;

    //     let params = ListQuizzesParams {
    //         category_id,
    //         difficulty: difficulty_enum,
    //         search_term,
    //         page,
    //         limit,
    //         sort_by: sort_by_enum,
    //         order: order_enum,
    //     };

    //     let (summaries, total_items) = self.quiz_repository.list_summaries(&params).await?;

    //     let quiz_dtos = summaries
    //         .into_iter()
    //         .map(|summary| QuizSummaryDto {
    //             id: summary.id,
    //             title: summary.title,
    //             difficulty: summary.difficulty.to_string(),
    //             question_count: summary.question_count,
    //             like_count: summary.like_count,
    //             comment_count: summary.comment_count,
    //             category_id: summary.category_id,
    //             category_name: summary.category_name,
    //         })
    //         .collect();

    //     let pagination_info = PaginationInfo {
    //         current_page: page,
    //         total_pages: (total_items as f64 / limit as f64).ceil() as u32,
    //         total_items,
    //         limit,
    //     };

    //     Ok(PaginatedQuizzesDto {
    //         pagination: pagination_info,
    //         data: quiz_dtos,
    //     })
    // }

    // pub async fn update_quiz(
    //     &self,
    //     quiz_id: i32,
    //     title: Option<String>,
    //     description: Option<String>,
    //     difficulty: Option<String>,
    //     category_id: Option<i32>,
    // ) -> ServiceResult<QuizDetailDto> {
    //     let difficulty_enum = difficulty
    //         .map(|d| {
    //             QuizDifficulty::from_string(&d)
    //                 .ok_or_else(|| ServiceError::bad_request("Invalid difficulty level"))
    //         })
    //         .transpose()?;

    //     let params = UpdateQuizParams {
    //         title,
    //         description,
    //         difficulty: difficulty_enum,
    //         category_id,
    //     };

    //     self.quiz_repository.update(quiz_id, &params).await?;
    //     self.get_quiz_detail(quiz_id).await
    // }

    // pub async fn delete_quiz(&self, quiz_id: i32) -> ServiceResult<()> {
    //     self.quiz_repository.delete(quiz_id).await?;
    //     Ok(())
    // }
}
