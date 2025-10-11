use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    QueryBuilder,
};

pub mod get;

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "quiz_difficulty", rename_all = "kebab-case")]
pub enum QuizDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizInfo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub difficulty: Option<QuizDifficulty>,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizQuery {
    pub category_id: Option<i32>,
    pub title_pattern: Option<String>,
    pub difficulty: Option<String>,
    pub completed_by: Option<i32>,
    pub page: i64,
    pub size: i64,
}

impl QuizQuery {
    fn apply_filters_for(&self, builder: &mut QueryBuilder<sqlx::Postgres>) {
        builder.push(" WHERE 1=1");

        if let Some(category_id) = self.category_id {
            builder.push(" AND q.category = ").push_bind(category_id);
        }

        if let Some(title) = &self.title_pattern {
            builder
                .push(" AND q.title ILIKE ")
                .push_bind(format!("%{}%", title));
        }

        if let Some(difficulty) = &self.difficulty {
            builder
                .push(" AND q.difficulty = ")
                .push_bind(difficulty.clone());
        }

        if let Some(user_id) = self.completed_by {
            let sub_query =
                " AND EXISTS (SELECT 1 FROM results r WHERE r.quiz_id = q.id AND r.user_id = ";
            builder.push(sub_query).push_bind(user_id).push(")");
        }
    }

    fn apply_pagination_for(&self, builder: &mut QueryBuilder<sqlx::Postgres>) {
        let page_size = self.size;
        let offset = (self.page - 1) * page_size;
        builder.push(" ORDER BY q.id DESC");
        builder.push(" LIMIT ").push_bind(page_size);
        builder.push(" OFFSET ").push_bind(offset);
    }
}
