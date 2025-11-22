use crate::models::user::DatabaseUser;

impl DatabaseUser {
    pub async fn delete_by_id(
        id: i32,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), crate::models::error::ModelError> {
        sqlx::query!(
            r#"DELETE FROM users
            WHERE usr_id = $1"#,
            id,
        )
        .execute(&mut *connection)
        .await?;
        Ok(())
    }
}
