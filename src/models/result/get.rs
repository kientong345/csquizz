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
        todo!()
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
