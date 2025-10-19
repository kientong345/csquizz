use sqlx::PgConnection;

use crate::models::{
    quiz::QuizInfo,
    result::QuizResultSummary,
    user::{FetchedUser, User, UserRole},
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
            QuizResultSummary::count_distinct_by_user_id(fetched_user.id, connection).await?;

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
            QuizResultSummary::count_distinct_by_user_id(fetched_user.id, connection).await?;

        Ok(User::create_from(
            fetched_user,
            quiz_created_count,
            quiz_completed_count,
        ))
    }
}
