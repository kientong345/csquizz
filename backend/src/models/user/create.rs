use std::str::FromStr;

use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    user::{DatabaseUser, UserCreateParams, UserRole},
};

impl DatabaseUser {
    pub async fn create_from(
        params: UserCreateParams,
        connection: &mut PgConnection,
    ) -> Result<DatabaseUser, ModelError> {
        let role = UserRole::from_str(&params.role.unwrap_or("user".to_string()))?;
        let id: i32 = sqlx::query_scalar!(
            r#"INSERT INTO users (usr_display_name, usr_email, usr_password_hash, usr_avatar_url, usr_role, usr_google_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING usr_id"#,
            params.display_name,
            params.email,
            params.password_hash,
            params.avatar_url,
            role as UserRole,
            params.google_id,
        ).fetch_one(&mut *connection).await?;

        Ok(DatabaseUser::get_by_id(id, connection).await?)
    }
}
