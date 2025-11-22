use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    user::{AuthUserUpdateParams, DatabaseUser, UserUpdateParams},
};

impl DatabaseUser {
    pub async fn update_by(
        params: &UserUpdateParams,
        connection: &mut PgConnection,
    ) -> Result<DatabaseUser, ModelError> {
        sqlx::query!(
            r#"UPDATE users
            SET
                usr_display_name = COALESCE($1, usr_display_name),
                usr_password_hash = COALESCE($2, usr_password_hash),
                usr_avatar_url = COALESCE($3, usr_avatar_url)
            WHERE usr_id = $4"#,
            params.display_name,
            params.password_hash,
            params.avatar_url,
            params.id,
        )
        .execute(&mut *connection)
        .await?;

        Ok(DatabaseUser::get_by_id(params.id, connection).await?)
    }

    pub async fn auth_update_by(
        id: i32,
        params: &AuthUserUpdateParams,
        connection: &mut PgConnection,
    ) -> Result<DatabaseUser, ModelError> {
        sqlx::query!(
            r#"UPDATE users
            SET
                usr_display_name = COALESCE($1, usr_display_name),
                usr_password_hash = COALESCE($2, usr_password_hash),
                usr_avatar_url = COALESCE($3, usr_avatar_url)
            WHERE usr_id = $4"#,
            params.display_name,
            params.password_hash,
            params.avatar_url,
            id,
        )
        .execute(&mut *connection)
        .await?;

        Ok(DatabaseUser::get_by_id(id, connection).await?)
    }
}
