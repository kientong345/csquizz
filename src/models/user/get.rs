use sqlx::PgConnection;

use crate::models::{
    pagination::{Page, Paginate},
    user::{User, UserMinimal, UserQuery, UserRole},
};

impl User {
    pub async fn get_by_id(id: i32, connection: &mut PgConnection) -> Result<User, sqlx::Error> {
        Ok(sqlx::query_as!(
            User,
            r#"SELECT id, username, avatar_url, email, role AS "role: UserRole"
            FROM users WHERE id = $1"#,
            id
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn get_by_email(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<User, sqlx::Error> {
        Ok(sqlx::query_as!(
            User,
            r#"SELECT id, username, avatar_url, email, role AS "role: UserRole"
            FROM users WHERE email = $1"#,
            email
        )
        .fetch_one(connection)
        .await?)
    }
}

impl Paginate<UserQuery> for UserMinimal {
    async fn page(
        query: &UserQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, sqlx::Error> {
        todo!()
    }
}
