use sqlx::{PgConnection, QueryBuilder};

use crate::models::{
    pagination::{Page, Paginate},
    quiz::{QuizDifficulty, QuizInfo, QuizQuery},
};

impl QuizInfo {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizInfo, sqlx::Error> {
        Ok(sqlx::query_as!(
            QuizInfo,
            r#"SELECT q.id, q.title, q.description, c.name AS category, q.difficulty AS "difficulty: QuizDifficulty", u.username AS created_by
            FROM quizzes AS q JOIN categories AS c ON q.category = c.id JOIN users AS u ON q.created_by = u.id
            WHERE q.id = $1"#, id
        ).fetch_one(connection).await?)
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
