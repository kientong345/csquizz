use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    quiz::DatabaseQuiz,
    submission_result::DatabaseSubmissionResult,
    user::{DatabaseUser, UserFullDetail, UserMinimal},
    user_follower::DatabaseUserFollower,
};

impl DatabaseUser {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<DatabaseUser, ModelError> {
        Ok(sqlx::query_as!(
            DatabaseUser,
            r#"SELECT
                usr_id AS id , usr_google_id AS google_id, usr_display_name AS display_name, usr_email AS email,
                usr_password_hash AS password_hash, usr_avatar_url AS avatar_url, usr_role AS "role: _", usr_created_at AS created_at
            FROM users
            WHERE usr_id = $1"#,
            id
        )
        .fetch_one(&mut *connection)
        .await?)
    }

    pub async fn get_by_email(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<DatabaseUser, ModelError> {
        Ok(sqlx::query_as!(
            DatabaseUser,
            r#"SELECT
                usr_id AS id , usr_google_id AS google_id, usr_display_name AS display_name, usr_email AS email,
                usr_password_hash AS password_hash, usr_avatar_url AS avatar_url, usr_role AS "role: _", usr_created_at AS created_at
            FROM users
            WHERE usr_email = $1"#,
            email
        )
        .fetch_one(&mut *connection)
        .await?)
    }

    pub async fn count(connection: &mut PgConnection) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(r#"SELECT COUNT(*) FROM users"#)
            .fetch_one(connection)
            .await?
            .unwrap_or(0))
    }

    pub async fn is_email_exist(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<bool, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM users WHERE usr_email = $1)"#,
            email
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(false))
    }

    pub async fn validate_login(
        email: &str,
        password: &str,
        connection: &mut PgConnection,
    ) -> Result<DatabaseUser, ModelError> {
        let user = DatabaseUser::get_by_email(email, connection).await?;
        let hash = user.password_hash.as_deref().unwrap_or("");
        if bcrypt::verify(password, hash).unwrap_or(false) {
            Ok(user)
        } else {
            Err(ModelError::WrongPasswordForEmail { email: user.email })
        }
    }
}

impl UserFullDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<UserFullDetail, ModelError> {
        let db_user = DatabaseUser::get_by_id(id, connection).await?;

        let quiz_created_count = DatabaseQuiz::count_by_creator_id(id, connection).await?;
        let quiz_completed_count =
            DatabaseSubmissionResult::passed_count_by_user_id(id, connection).await?;
        let follower_count = DatabaseUserFollower::follower_count_for(id, connection).await?;

        Ok(UserFullDetail {
            id,
            display_name: db_user.display_name,
            email: db_user.email,
            avatar_url: db_user.avatar_url,
            role: db_user.role.to_string(),
            created_at: db_user.created_at.map(|dt| dt.to_rfc3339()),
            quiz_completed_count,
            quiz_created_count,
            follower_count,
        })
    }

    pub async fn get_by_email(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<UserFullDetail, ModelError> {
        let db_user = DatabaseUser::get_by_email(email, connection).await?;

        let quiz_created_count = DatabaseQuiz::count_by_creator_id(db_user.id, connection).await?;
        let quiz_completed_count =
            DatabaseSubmissionResult::passed_count_by_user_id(db_user.id, connection).await?;
        let follower_count =
            DatabaseUserFollower::follower_count_for(db_user.id, connection).await?;

        Ok(UserFullDetail {
            id: db_user.id,
            display_name: db_user.display_name,
            email: db_user.email,
            avatar_url: db_user.avatar_url,
            role: db_user.role.to_string(),
            created_at: db_user.created_at.map(|dt| dt.to_rfc3339()),
            quiz_completed_count,
            quiz_created_count,
            follower_count,
        })
    }
}

impl UserMinimal {
    /// Get the list of users followed by a specific user
    pub async fn list_users_followed_by(
        user_id: &i32,
        connection: &mut PgConnection,
    ) -> Result<Vec<UserMinimal>, ModelError> {
        Ok(sqlx::query_as!(
            UserMinimal,
            r#"SELECT u.usr_id AS id, u.usr_display_name AS display_name, u.usr_avatar_url AS avatar_url
            FROM user_followers AS ufl
            INNER JOIN users AS u ON ufl.ufl_followed_id = u.usr_id
            WHERE ufl.ufl_follower_id = $1"#,
            user_id
        ).fetch_all(connection).await?)
    }
}

#[cfg(feature = "local")]
#[cfg(test)]
mod tests {
    use sqlx::{Postgres, pool::PoolConnection};

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
