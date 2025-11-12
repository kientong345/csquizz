use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Enum đại diện cho vai trò của người dùng trong hệ thống.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
}

/// Struct đại diện cho một người dùng trong database.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    #[serde(rename = "id")]
    pub usr_id: i32,

    #[serde(rename = "googleId")]
    pub usr_google_id: Option<String>,

    #[serde(rename = "displayName")]
    pub usr_display_name: String,

    #[serde(rename = "email")]
    pub usr_email: String,

    #[serde(skip_serializing)] // Không bao giờ gửi password hash ra ngoài
    pub usr_password_hash: Option<String>,

    #[serde(rename = "avatarUrl")]
    pub usr_avatar_url: Option<String>,

    #[serde(rename = "role")]
    pub usr_role: UserRole,

    #[serde(rename = "createdAt")]
    pub usr_created_at: Option<DateTime<Utc>>,
}
