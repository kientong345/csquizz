use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, QueryBuilder};

use crate::models::{
    pagination::{Page, Paginate},
    quiz::QuizInfo,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizQuery {
    pub category_id: Option<i32>,
    pub title_pattern: Option<String>,
    pub difficulty: Option<String>,
    pub created_by: Option<i32>,
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

        if let Some(user_id) = self.created_by {
            builder.push(" AND q.created_by = ").push_bind(user_id);
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

impl Paginate<QuizQuery> for QuizInfo {
    async fn page(
        query: &QuizQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, sqlx::Error> {
        let mut count_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "SELECT count(q.id) FROM quizzes AS q JOIN categories AS c ON q.category = c.id",
        );
        query.apply_filters_for(&mut count_builder);

        let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            r#"SELECT q.id, q.title, q.description, c.name AS category, q.difficulty, u.username AS created_by
            FROM quizzes AS q
            JOIN categories AS c ON q.category = c.id
            JOIN users AS u ON q.created_by = u.id"#,
        );
        query.apply_filters_for(&mut query_builder);
        query.apply_pagination_for(&mut query_builder);

        let total_items: i64 = count_builder
            .build_query_scalar()
            .fetch_one(&mut *connection)
            .await?;

        let items: Vec<QuizInfo> = query_builder.build_query_as().fetch_all(connection).await?;

        Ok(Page::create_from(items, total_items, query.size))
    }
}
