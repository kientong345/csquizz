use serde_json::{Value, json};
use sqlx::PgConnection;

use crate::{
    models::{
        answer::Answer,
        comment::{CommentDetail, DatabaseComment},
        input_dto::{
            comment::{CommentCreateParamsDto, CommentPaginateParamsDto},
            question::{
                QuestionCreateParamsDto, QuestionPaginateParamsDto, QuestionUpdateParamsDto,
            },
            quiz::QuizUpdateParamsDto,
            quiz_question::QuizQuestionCreateParamsDto,
            submission::QuizSubmissionParamsDto,
        },
        like::DatabaseQuizLike,
        pagination::Paginate,
        question::{DatabaseQuestion, QuestionPrivateData, QuestionPublicData},
        quiz::{DatabaseQuiz, QuizDetail, QuizMinimal, QuizPaginateParams},
        quiz_composition::{QuizComment, QuizPublicQuestion},
        submission_result::DatabaseSubmissionResult,
    },
    services::{error::ServiceError, evaluator::Evaluator},
};

#[derive(Clone)]
pub struct QuizService;

impl QuizService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_quizzes_page(
        &self,
        conn: &mut PgConnection,
        params: &QuizPaginateParams,
    ) -> Result<Value, ServiceError> {
        let page = QuizMinimal::page(params, conn).await?;
        Ok(json!(page))
    }

    pub async fn get_quiz_with_comments(
        &self,
        conn: &mut PgConnection,
        id: i32,
        params: &CommentPaginateParamsDto,
    ) -> Result<Value, ServiceError> {
        let metadata = QuizDetail::get_by_id(id, conn).await?;
        let data = CommentDetail::page(&params.clone().bind(id), conn).await?;
        Ok(json!(QuizComment { metadata, data }))
    }

    pub async fn get_quiz_with_questions(
        &self,
        conn: &mut PgConnection,
        id: i32,
        params: &QuestionPaginateParamsDto,
    ) -> Result<Value, ServiceError> {
        let metadata = QuizDetail::get_by_id(id, conn).await?;
        let data = QuestionPrivateData::page(&params.clone().bind(id), conn)
            .await?
            .try_map_into::<QuestionPublicData>()?;
        Ok(json!(QuizPublicQuestion { metadata, data }))
    }

    pub async fn create_quiz_with_questions(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        payload: &QuizQuestionCreateParamsDto,
    ) -> Result<(), ServiceError> {
        let quiz_id = DatabaseQuiz::create_from(&payload.quiz_params.clone().bind(user_id), conn)
            .await?
            .id;

        for params in &payload.questions_params {
            DatabaseQuestion::create_from(&params.clone().bind(quiz_id), conn).await?;
        }
        Ok(())
    }

    pub async fn update_quiz_metadata(
        &self,
        conn: &mut PgConnection,
        id: i32,
        payload: &QuizUpdateParamsDto,
    ) -> Result<(), ServiceError> {
        DatabaseQuiz::update_by(&payload.clone().bind(id), conn).await?;
        Ok(())
    }

    pub async fn delete_quiz(&self, conn: &mut PgConnection, id: i32) -> Result<(), ServiceError> {
        DatabaseQuiz::delete_by(id, conn).await?;
        Ok(())
    }

    pub async fn like_quiz(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        id: i32,
    ) -> Result<(), ServiceError> {
        DatabaseQuizLike::create_from(user_id, id, conn).await?;
        Ok(())
    }

    pub async fn comment_quiz(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        id: i32,
        payload: &CommentCreateParamsDto,
    ) -> Result<(), ServiceError> {
        DatabaseComment::create_from(&payload.clone().bind(user_id, id), conn).await?;
        Ok(())
    }

    pub async fn add_question(
        &self,
        conn: &mut PgConnection,
        id: i32,
        payload: &QuestionCreateParamsDto,
    ) -> Result<(), ServiceError> {
        DatabaseQuestion::create_from(&payload.clone().bind(id), conn).await?;
        Ok(())
    }

    pub async fn update_question(
        &self,
        conn: &mut PgConnection,
        question_id: i32,
        payload: &QuestionUpdateParamsDto,
    ) -> Result<(), ServiceError> {
        DatabaseQuestion::update_by(&payload.clone().bind(question_id), conn).await?;
        Ok(())
    }

    pub async fn delete_question(
        &self,
        conn: &mut PgConnection,
        question_id: i32,
    ) -> Result<(), ServiceError> {
        DatabaseQuestion::delete_by(question_id, conn).await?;
        Ok(())
    }

    pub async fn submit_quiz(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        id: i32,
        payload: &QuizSubmissionParamsDto,
    ) -> Result<(), ServiceError> {
        let submission = payload.clone().bind(user_id, id);

        let (submission_result_summary, evaluated_answers) =
            Evaluator::evaluate(&submission, conn).await?;

        let result_id = DatabaseSubmissionResult::create_from(&submission_result_summary, conn)
            .await?
            .id;

        for evaluated_answer in evaluated_answers {
            Answer::create_from(&evaluated_answer.bind(result_id), conn).await?;
        }
        Ok(())
    }
}
