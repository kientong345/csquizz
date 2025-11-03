use sqlx::PgConnection;

use crate::models::{
    auth::AuthenticationType,
    error::ModelError,
    user::{FetchedUser, UserFullDetail, UserRole},
};

impl UserFullDetail {
    pub async fn create_from(
        auth_type: AuthenticationType,
        connection: &mut PgConnection,
    ) -> Result<UserFullDetail, ModelError> {
        match auth_type {
            AuthenticationType::WithPassword(registration) => {
                let password_hash = bcrypt::hash(registration.password, bcrypt::DEFAULT_COST)?;

                let fetched_user = sqlx::query_as!(
                    FetchedUser,
                    r#"INSERT INTO users (display_name, password_hash, email) VALUES ($1, $2, $3)
                    RETURNING id, display_name, avatar_url, email, role AS "role: UserRole", password_hash, google_id"#,
                    registration.display_name,
                    password_hash,
                    registration.email,
                )
                .fetch_one(connection)
                .await?;

                Ok(UserFullDetail::build_from(fetched_user, 0, 0))
            }
            AuthenticationType::OAuth(oauth_payload) => {
                let fetched_user = sqlx::query_as!(
                    FetchedUser,
                    r#"INSERT INTO users (google_id, display_name, email) VALUES ($1, $2, $3)
                    RETURNING id, display_name, avatar_url, email, role AS "role: UserRole", password_hash, google_id"#,
                    oauth_payload.google_id,
                    oauth_payload.display_name,
                    oauth_payload.email,
                )
                .fetch_one(connection)
                .await?;

                Ok(UserFullDetail::build_from(fetched_user, 0, 0))
            }
        }
    }
}
