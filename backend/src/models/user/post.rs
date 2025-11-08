use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    user::{FetchedUser, UserFullDetail, UserRole},
};

#[derive(Debug)]
pub struct PostUser {
    pub google_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<String>, // "user" || "admin"
}

impl UserFullDetail {
    pub async fn create_from(
        data: PostUser,
        connection: &mut PgConnection,
    ) -> Result<UserFullDetail, ModelError> {
        let fetched_user = sqlx::query_as!(
            FetchedUser,
            r#"INSERT INTO users (google_id, display_name, email, password_hash, avatar_url, role)
            VALUES ($1, $2, $3, $4, $5, $6::text::user_role)
            RETURNING id, display_name, avatar_url, email, role AS "role: UserRole", password_hash, google_id"#,
            data.google_id,
            data.display_name,
            data.email,
            data.password_hash,
            data.avatar_url,
            data.role,
        ).fetch_one(connection).await?;
        Ok(UserFullDetail::build_from(fetched_user, 0, 0))
    }
}
