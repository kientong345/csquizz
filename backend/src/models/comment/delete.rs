use sqlx::PgConnection;

use crate::models::{comment::DatabaseComment, error::ModelError};

impl DatabaseComment {
    pub async fn delete_by(
        comment_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"DELETE FROM comments
            WHERE cmt_id = $1"#,
            comment_id
        )
        .execute(connection)
        .await?;
        Ok(())
    }
}
