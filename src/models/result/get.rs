use sqlx::PgConnection;

use crate::models::{
    pagination::{Page, Paginate},
    result::{QuizResult, QuizResultDetail, QuizResultQuery},
};

impl Paginate<QuizResultQuery> for QuizResult {
    async fn page(
        query: &QuizResultQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, sqlx::Error> {
        let total_items = QuizResult::count_by_user_id(query.user_id, connection).await?;
        let offset = (query.page.saturating_sub(1)) * query.size;

        let items = sqlx::query_as!(
            QuizResult,
            r#"SELECT r.id, q.title AS quiz_title, r.score, r.total_questions, r.correct_answers
            FROM results AS r JOIN quizzes AS q ON r.quiz_id = q.id LIMIT $1 OFFSET $2"#,
            query.size,
            offset
        )
        .fetch_all(connection)
        .await?;

        Ok(Page::create_from(items, total_items, query.size))
    }
}

impl QuizResultDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizResultDetail, sqlx::Error> {
        todo!()
    }
}
