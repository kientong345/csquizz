use crate::models::user_follower::DatabaseUserFollower;

impl DatabaseUserFollower {
    pub async fn delete_by(
        follower_id: i32,
        followed_id: i32,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), crate::models::error::ModelError> {
        sqlx::query!(
            r#"DELETE FROM user_followers WHERE ufl_follower_id = $1 AND ufl_followed_id = $2"#,
            follower_id,
            followed_id
        )
        .execute(&mut *connection)
        .await?;

        Ok(())
    }
}
