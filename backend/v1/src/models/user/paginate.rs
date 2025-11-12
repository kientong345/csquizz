use serde::Deserialize;
use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    user::{OrderType, UserFullDetail, UserPubInfo},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserQuery {
    pub order_by: OrderType,
    pub page: i64,
    pub size: i64,
}

impl Paginate<UserQuery> for UserPubInfo {
    async fn page(
        query: &UserQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let mut query_str = String::from(
            r#"SELECT
                u.id, u.username, u.avatar_url, u.role AS "role: UserRole",
                COALESCE(quiz_counts.count, 0) AS "quiz_created_count: i64",
                COALESCE(result_counts.count, 0) AS "quiz_completed_count: i64"
            FROM users AS u
            LEFT JOIN
                (SELECT created_by, COUNT(id) AS count FROM quizzes GROUP BY created_by) AS quiz_counts
                ON u.id = quiz_counts.created_by
            LEFT JOIN
                (SELECT user_id, COUNT(id) AS count FROM results GROUP BY user_id) AS result_counts
                ON u.id = result_counts.user_id"#,
        );

        match query.order_by {
            OrderType::MostCreated => {
                query_str.push_str(" ORDER BY quiz_created_count DESC");
            }
            OrderType::MostSolved => {
                query_str.push_str(" ORDER BY quiz_completed_count DESC");
            }
        }

        let total_items = UserFullDetail::count(connection).await?;
        let offset = (query.page.saturating_sub(1)) * query.size;
        query_str.push_str(" LIMIT $1 OFFSET $2");

        let items: Vec<UserPubInfo> = sqlx::query_as(&query_str)
            .bind(query.size)
            .bind(offset)
            .fetch_all(connection)
            .await?;

        Ok(Page::build_from(items, total_items, query.size))
    }
}
