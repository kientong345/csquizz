use std::str::FromStr;

use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    quiz::{DatabaseQuiz, QuizCreateParams, QuizDifficulty},
};

impl DatabaseQuiz {
    pub async fn create_from(
        params: &QuizCreateParams,
        connection: &mut PgConnection,
    ) -> Result<DatabaseQuiz, ModelError> {
        let difficulty = if let Some(diff) = &params.difficulty {
            Some(QuizDifficulty::from_str(diff)?)
        } else {
            None
        };

        let quiz = sqlx::query_as!(
            DatabaseQuiz,
            r#"INSERT INTO quizzes (qz_title, qz_description, qz_difficulty, qz_category_id, qz_creator_id, qz_pass_score)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                qz_id AS id, qz_title AS title, qz_description AS description, qz_difficulty as "difficulty: _", qz_category_id AS category_id,
                qz_pass_score AS pass_score, qz_creator_id AS creator_id, qz_created_at AS created_at, qz_updated_at AS updated_at"#,
            params.title,
            params.description,
            difficulty as Option<QuizDifficulty>,
            params.category_id,
            params.creator_id,
            params.pass_score,
        )
        .fetch_one(connection)
        .await?;

        Ok(quiz)
    }
}
