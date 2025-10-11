use sqlx::PgConnection;

use crate::models::{
    auth::SignupMethod,
    user::{User, UserRole},
};

impl User {
    pub async fn create(
        signup_method: SignupMethod,
        connection: &mut PgConnection,
    ) -> Result<User, sqlx::Error> {
        match signup_method {
            SignupMethod::WithPassword(registration) => {
                let password_hash = bcrypt::hash(registration.password, bcrypt::DEFAULT_COST)
                    .expect("hashing failed");

                Ok(sqlx::query_as!(
                    User,
                    r#"INSERT INTO users (username, password_hash, email) VALUES ($1, $2, $3)
                    RETURNING id, username, avatar_url, email, role AS "role: UserRole""#,
                    registration.username,
                    password_hash,
                    registration.email,
                )
                .fetch_one(connection)
                .await?)
            }
            SignupMethod::OAuth(oauth_payload) => Ok(sqlx::query_as!(
                User,
                r#"INSERT INTO users (google_id, username, email) VALUES ($1, $2, $3)
                    RETURNING id, username, avatar_url, email, role AS "role: UserRole""#,
                oauth_payload.google_id,
                oauth_payload.username,
                oauth_payload.email,
            )
            .fetch_one(connection)
            .await?),
        }
    }
}
