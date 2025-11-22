use crate::models::category::{Category, CategoryUpdateParams};

impl Category {
    pub async fn update_by(
        params: &CategoryUpdateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<Category, crate::models::error::ModelError> {
        Ok(sqlx::query_as!(
            Category,
            r#"UPDATE categories
            SET
                cat_name = COALESCE($2, cat_name),
                cat_image_url = COALESCE($3, cat_image_url),
                cat_description = COALESCE($4, cat_description)
            WHERE cat_id = $1
            RETURNING cat_id AS id, cat_name AS name, cat_image_url AS image_url, cat_description AS description"#,
            params.id,
            params.name,
            params.image_url,
            params.description
        )
        .fetch_one(connection)
        .await?)
    }
}
