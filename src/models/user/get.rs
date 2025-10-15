use sqlx::PgConnection;

use crate::models::{
    pagination::{Page, Paginate},
    quiz::QuizInfo,
    result::QuizResult,
    user::{FetchedUser, OrderType, User, UserMinimal, UserQuery, UserRole},
};

impl User {
    pub async fn get_by_id(id: i32, connection: &mut PgConnection) -> Result<User, sqlx::Error> {
        let fetched_user = sqlx::query_as!(
            FetchedUser,
            r#"SELECT id , username, avatar_url, email, role AS "role: UserRole", password_hash, google_id
            FROM users WHERE id = $1"#,
            id
        )
        .fetch_one(&mut *connection)
        .await?;

        let quiz_created_count = QuizInfo::count_by_creator_id(fetched_user.id, connection).await?;
        let quiz_completed_count =
            QuizResult::count_by_user_id(fetched_user.id, connection).await?;

        Ok(User::create_from(
            fetched_user,
            quiz_created_count,
            quiz_completed_count,
        ))
    }

    pub async fn get_by_email(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<User, sqlx::Error> {
        let fetched_user = sqlx::query_as!(
            FetchedUser,
            r#"SELECT id , username, avatar_url, email, role AS "role: UserRole", password_hash, google_id
            FROM users WHERE email = $1"#,
            email
        )
        .fetch_one(&mut *connection)
        .await?;

        let quiz_created_count = QuizInfo::count_by_creator_id(fetched_user.id, connection).await?;
        let quiz_completed_count =
            QuizResult::count_by_user_id(fetched_user.id, connection).await?;

        Ok(User::create_from(
            fetched_user,
            quiz_created_count,
            quiz_completed_count,
        ))
    }
}

impl Paginate<UserQuery> for UserMinimal {
    async fn page(
        query: &UserQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, sqlx::Error> {
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

        let total_items = User::count(connection).await?;
        let offset = (query.page.saturating_sub(1)) * query.size;
        query_str.push_str(" LIMIT $1 OFFSET $2");

        let items: Vec<UserMinimal> = sqlx::query_as(&query_str)
            .bind(query.size)
            .bind(offset)
            .fetch_all(connection)
            .await?;

        Ok(Page::create_from(items, total_items, query.size))
    }
}
