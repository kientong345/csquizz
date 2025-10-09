use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

use crate::models::paginate::Paginate;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizInfo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub difficulty: Option<String>,
    pub created_by: Option<String>,
}

impl QuizInfo {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizInfo, sqlx::Error> {
        Ok(sqlx::query_as!(
            QuizInfo,
            r#"SELECT q.id, q.title, q.description, c.name AS category, q.difficulty, u.username AS created_by
            FROM quizzes AS q JOIN categories AS c ON q.category = c.id JOIN users AS u ON q.created_by = u.id
            WHERE q.id = $1"#, id
        ).fetch_one(connection).await?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizQuery {
    pub category_id: i32,
    pub title_pattern: Option<String>,
    pub difficulty: Option<String>,
    pub completed_by: Option<i32>,
    pub page: i32,
    pub size: i32,
}

impl Paginate<QuizQuery> for QuizInfo {
    async fn page(
        query: &QuizQuery,
        connection: &mut PgConnection,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        todo!()
    }
}
