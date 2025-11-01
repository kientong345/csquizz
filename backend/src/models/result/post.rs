use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    result::{QuestionResult, QuizResult, QuizResultSummary, TmpQuizResult, UserAnswer},
    submission::{EvaluatedQuestionResult, EvaluatedQuizResultSummary},
};

impl QuizResultSummary {
    async fn create_from(
        user_id: i32,
        data: &EvaluatedQuizResultSummary,
        connection: &mut PgConnection,
    ) -> Result<QuizResultSummary, ModelError> {
        let id = sqlx::query_scalar!(
            r#"INSERT INTO results (user_id, quiz_id, score, total_questions, correct_answers)
            VALUES ($1, $2, $3, $4, $5) RETURNING id"#,
            user_id,
            data.quiz_id,
            data.score,
            data.total_questions,
            data.correct_answers,
        )
        .fetch_one(&mut *connection)
        .await?;

        Ok(QuizResultSummary::get_by_id(id, connection).await?)
    }
}

impl QuestionResult {
    async fn create_from(
        data: &EvaluatedQuestionResult,
        connection: &mut PgConnection,
    ) -> Result<QuestionResult, ModelError> {
        let answer_data = match &data.answer_data {
            UserAnswer::MultipleChoiceAnswer(choices) => {
                serde_json::json!(choices)
            }
            UserAnswer::SingleChoiceAnswer(choice) => {
                serde_json::json!(choice)
            }
            UserAnswer::TextEntryAnswer(entry) => {
                serde_json::json!(entry)
            }
        };

        let id = sqlx::query_scalar!(
            r#"INSERT INTO user_answers (result_id, question_id, answer_data)
            VALUES ($1, $2, $3) RETURNING id"#,
            data.result_id,
            data.question_id,
            answer_data,
        )
        .fetch_one(&mut *connection)
        .await?;

        Ok(QuestionResult::get_by_id(id, connection).await?)
    }
}

impl QuizResult {
    pub async fn create_from(
        user_id: i32,
        summary_data: &EvaluatedQuizResultSummary,
        result_data: &Vec<EvaluatedQuestionResult>,
        connection: &mut PgConnection,
    ) -> Result<QuizResult, ModelError> {
        let summary = QuizResultSummary::create_from(user_id, summary_data, connection).await?;
        let mut result = Vec::new();
        for data in result_data {
            result.push(QuestionResult::create_from(data, connection).await?);
        }
        Ok(QuizResult { summary, result })
    }
}

impl TmpQuizResult {
    pub async fn create_from(
        summary_data: &EvaluatedQuizResultSummary,
        result_data: &Vec<EvaluatedQuestionResult>,
        connection: &mut PgConnection,
    ) -> Result<TmpQuizResult, ModelError> {
        todo!()
    }
}
