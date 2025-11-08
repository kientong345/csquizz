use serde::Deserialize;
use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    quiz::{QuizDifficulty, QuizMetadata},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostQuizMetadata {
    pub title: String,
    pub description: Option<String>,
    pub category_id: i32,
    pub difficulty: Option<QuizDifficulty>,
    pub creator_id: Option<i32>,
}

impl QuizMetadata {
    pub async fn create_from(
        data: PostQuizMetadata,
        connection: &mut PgConnection,
    ) -> Result<QuizMetadata, ModelError> {
        let difficulty = if let Some(diff_type) = data.difficulty {
            match diff_type {
                QuizDifficulty::Easy => Some(String::from("easy")),
                QuizDifficulty::Medium => Some(String::from("medium")),
                QuizDifficulty::Hard => Some(String::from("hard")),
            }
        } else {
            None
        };

        let id = sqlx::query!(
            r#"INSERT INTO quizzes (title, description, category, difficulty, created_by)
            VALUES ($1, $2, $3, $4::text::quiz_difficulty, $5) RETURNING id"#,
            data.title,
            data.description,
            data.category_id,
            difficulty,
            data.creator_id,
        )
        .fetch_one(&mut *connection)
        .await?
        .id;

        Ok(QuizMetadata::get_by_id(id, connection).await?)
    }
}
