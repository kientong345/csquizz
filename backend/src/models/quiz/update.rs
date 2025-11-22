use std::str::FromStr;

use sqlx::PgConnection;

use crate::models::quiz::{DatabaseQuiz, QuizDifficulty, QuizUpdateParams};

impl DatabaseQuiz {
    pub async fn update_by(
        params: &QuizUpdateParams,
        connection: &mut PgConnection,
    ) -> Result<DatabaseQuiz, crate::models::error::ModelError> {
        let difficulty = if let Some(diff) = &params.difficulty {
            Some(QuizDifficulty::from_str(diff)?)
        } else {
            None
        };

        let quiz = sqlx::query_as!(
            DatabaseQuiz,
            r#"UPDATE quizzes
            SET
                qz_title = COALESCE($1, qz_title),
                qz_description = COALESCE($2, qz_description),
                qz_difficulty = COALESCE($3, qz_difficulty),
                qz_category_id = COALESCE($4, qz_category_id),
                qz_pass_score = COALESCE($5, qz_pass_score),
                qz_updated_at = NOW()
            WHERE qz_id = $6
            RETURNING
                qz_id AS id, qz_title AS title, qz_description AS description, qz_difficulty as "difficulty: _", qz_category_id AS category_id,
                qz_pass_score AS pass_score, qz_creator_id AS creator_id, qz_created_at AS created_at, qz_updated_at AS updated_at"#,
            params.title,
            params.description,
            difficulty as Option<QuizDifficulty>,
            params.category_id,
            params.pass_score,
            params.id,
        )
        .fetch_one(connection)
        .await?;

        Ok(quiz)
    }
}
