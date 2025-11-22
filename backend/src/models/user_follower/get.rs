use sqlx::PgConnection;

use crate::models::{error::ModelError, user_follower::DatabaseUserFollower};

impl DatabaseUserFollower {
    pub async fn follower_count_for(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM user_followers WHERE ufl_followed_id = $1"#,
            user_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }

    pub async fn followed_count_for(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM user_followers WHERE ufl_follower_id = $1"#,
            user_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }
}
