use crate::models::user_follower::DatabaseUserFollower;

impl DatabaseUserFollower {
    pub async fn create_from(
        follower_id: i32,
        followed_id: i32,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), crate::models::error::ModelError> {
        sqlx::query!(
            r#"INSERT INTO user_followers (ufl_follower_id, ufl_followed_id) VALUES ($1, $2)"#,
            follower_id,
            followed_id
        )
        .execute(&mut *connection)
        .await?;

        Ok(())
    }
}
