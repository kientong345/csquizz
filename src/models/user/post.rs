use sqlx::PgConnection;

use crate::models::{
    auth::SignupMethod,
    user::{FetchedUser, User, UserRole},
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

                let fetched_user = sqlx::query_as!(
                    FetchedUser,
                    r#"INSERT INTO users (username, password_hash, email) VALUES ($1, $2, $3)
                    RETURNING id, username, avatar_url, email, role AS "role: UserRole", password_hash, google_id"#,
                    registration.username,
                    password_hash,
                    registration.email,
                )
                .fetch_one(connection)
                .await?;

                Ok(User::create_from(fetched_user, 0, 0))
            }
            SignupMethod::OAuth(oauth_payload) => {
                let fetched_user = sqlx::query_as!(
                    FetchedUser,
                    r#"INSERT INTO users (google_id, username, email) VALUES ($1, $2, $3)
                    RETURNING id, username, avatar_url, email, role AS "role: UserRole", password_hash, google_id"#,
                    oauth_payload.google_id,
                    oauth_payload.username,
                    oauth_payload.email,
                )
                .fetch_one(connection)
                .await?;

                Ok(User::create_from(fetched_user, 0, 0))
            }
        }
    }
}
