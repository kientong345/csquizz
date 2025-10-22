use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    quiz::QuizInfo,
    result::QuizResultSummary,
    user::{FetchedUser, UserFullDetail, UserPubInfo, UserRole},
};

impl UserPubInfo {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<UserPubInfo, ModelError> {
        let fetched_user = sqlx::query_as!(
            FetchedUser,
            r#"SELECT id , display_name, avatar_url, email, role AS "role: UserRole", password_hash, google_id
            FROM users WHERE id = $1"#,
            id
        )
        .fetch_one(&mut *connection)
        .await?;

        let quiz_created_count = QuizInfo::count_by_creator_id(fetched_user.id, connection).await?;
        let quiz_completed_count =
            QuizResultSummary::count_distinct_by_user_id(fetched_user.id, connection).await?;

        Ok(UserPubInfo {
            id: fetched_user.id,
            display_name: fetched_user.display_name,
            avatar_url: fetched_user.avatar_url,
            role: fetched_user.role,
            quiz_completed_count,
            quiz_created_count,
        })
    }
}

impl UserFullDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<UserFullDetail, ModelError> {
        let fetched_user = sqlx::query_as!(
            FetchedUser,
            r#"SELECT id , display_name, avatar_url, email, role AS "role: UserRole", password_hash, google_id
            FROM users WHERE id = $1"#,
            id
        )
        .fetch_one(&mut *connection)
        .await?;

        let quiz_created_count = QuizInfo::count_by_creator_id(fetched_user.id, connection).await?;
        let quiz_completed_count =
            QuizResultSummary::count_distinct_by_user_id(fetched_user.id, connection).await?;

        Ok(UserFullDetail::build_from(
            fetched_user,
            quiz_created_count,
            quiz_completed_count,
        ))
    }

    pub async fn get_by_email(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<UserFullDetail, ModelError> {
        let fetched_user = sqlx::query_as!(
            FetchedUser,
            r#"SELECT id , display_name, avatar_url, email, role AS "role: UserRole", password_hash, google_id
            FROM users WHERE email = $1"#,
            email
        )
        .fetch_one(&mut *connection)
        .await?;

        let quiz_created_count = QuizInfo::count_by_creator_id(fetched_user.id, connection).await?;
        let quiz_completed_count =
            QuizResultSummary::count_distinct_by_user_id(fetched_user.id, connection).await?;

        Ok(UserFullDetail::build_from(
            fetched_user,
            quiz_created_count,
            quiz_completed_count,
        ))
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{pool::PoolConnection, Postgres};

    use crate::{
        database::load_sample,
        models::user::{UserFullDetail, UserPubInfo, UserRole},
    };

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_minimal_users(mut conn: PoolConnection<Postgres>) {
        load_sample(&mut conn).await;

        let user1 = UserPubInfo::get_by_id(1, &mut conn).await.unwrap();
        let user2 = UserPubInfo::get_by_id(2, &mut conn).await.unwrap();

        assert_eq!(user1.display_name, "bocchi_the_dev".to_string());
        assert_eq!(user1.quiz_completed_count, 0);
        assert_eq!(user2.display_name, "super_user".to_string());
        assert_eq!(user2.quiz_created_count, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_users(mut conn: PoolConnection<Postgres>) {
        load_sample(&mut conn).await;

        let user1 = UserFullDetail::get_by_id(1, &mut conn).await.unwrap();
        let user2 = UserFullDetail::get_by_id(2, &mut conn).await.unwrap();

        assert_eq!(user1.google_id, None);
        assert_eq!(user1.email, "bocchi345@gmail.com".to_string());
        assert_eq!(user1.pub_info.role, UserRole::Admin);
        assert_eq!(user2.pub_info.display_name, "super_user".to_string());
        assert_eq!(user2.pub_info.role, UserRole::User);
    }
}
