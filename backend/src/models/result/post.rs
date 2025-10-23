use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    result::{AnswerResultType, QuizResult},
};

impl QuizResult {
    pub async fn store(
        &self,
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        let result_id = sqlx::query!(
            r#"INSERT INTO results (user_id, quiz_id, score, total_questions, correct_answers)
            VALUES ($1, $2, $3, $4, $5) RETURNING id"#,
            user_id,
            self.summary.quiz_id,
            self.summary.score as i32,
            self.summary.total_questions,
            self.summary.correct_answers,
        )
        .fetch_one(&mut *connection)
        .await?
        .id;

        for answer_results in &self.result {
            sqlx::query!(
                r#"INSERT INTO user_answers (result_id, question_id)
                VALUES ($1, $2)"#,
                result_id,
                answer_results.question_id,
            )
            .execute(&mut *connection)
            .await?;
            match &answer_results.answer {
                AnswerResultType::ChoicesResult(choice_results) => {
                    for (selected_option, is_correct) in choice_results {
                        sqlx::query!(
                            r#"INSERT INTO user_answers (selected_option, is_correct)
                            VALUES ($1, $2)"#,
                            selected_option,
                            is_correct,
                        )
                        .execute(&mut *connection)
                        .await?;
                    }
                }
                AnswerResultType::TextResult(entried_text, is_correct) => {
                    sqlx::query!(
                        r#"INSERT INTO user_answers (entried_text, is_correct)
                        VALUES ($1, $2)"#,
                        entried_text,
                        is_correct,
                    )
                    .execute(&mut *connection)
                    .await?;
                }
                AnswerResultType::InvalidResult => (),
            };
        }

        Ok(())
    }
}
