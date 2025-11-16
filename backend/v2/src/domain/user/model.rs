use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Debug, Clone, Copy)]
pub enum UserSortField {
    QuizCompletedCount,
    QuizCreatedCount,
}

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub google_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserMinimal {
    pub id: i32,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub quiz_completed_count: i64,
    pub quiz_created_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserDetail {
    pub user: User,
    pub quiz_completed_count: i64,
    pub quiz_created_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct CreateUserParams {
    pub google_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
}

#[derive(Debug, Clone, FromRow)]
pub struct UpdateUserParams {
    pub id: i32,
    pub display_name: Option<String>,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<UserRole>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserQuery {
    pub name_pattern: String,
    pub page: i32,
    pub limit: i32,
    pub sort_by: UserSortField,
}
