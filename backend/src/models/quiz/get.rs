use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    quiz::{QuizDifficulty, QuizInfo},
};

impl QuizInfo {
    pub async fn get_by_id(id: i32, connection: &mut PgConnection) -> Result<QuizInfo, ModelError> {
        Ok(sqlx::query_as!(
            QuizInfo,
            r#"SELECT q.id, q.title, q.description, c.name AS category, q.difficulty AS "difficulty: QuizDifficulty", u.display_name AS created_by
            FROM quizzes AS q JOIN categories AS c ON q.category = c.id JOIN users AS u ON q.created_by = u.id
            WHERE q.id = $1"#, id
        ).fetch_one(connection).await?)
    }
}
