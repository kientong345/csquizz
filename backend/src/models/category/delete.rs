use crate::models::category::Category;

impl Category {
    pub async fn delete_by(
        category_id: i32,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), crate::models::error::ModelError> {
        sqlx::query!(r#"DELETE FROM categories WHERE cat_id = $1"#, category_id)
            .execute(connection)
            .await?;

        Ok(())
    }
}
