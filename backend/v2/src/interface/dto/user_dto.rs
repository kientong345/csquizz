use serde::{Deserialize, Serialize};
use crate::domain::models::user::{User, UserRole};

// --- Query Structs ---

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct RegisterUserDto {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleLoginDto {
    #[serde(rename = "googleToken")]
    pub google_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserProfileDto {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleDto {
    pub role: UserRole,
}


// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct LoginResponseDto {
    #[serde(rename = "tokenType")]
    pub token_type: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: i32,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub email: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    pub role: UserRole,
    #[serde(rename = "createdAt")]
    pub created_at: String, // Trả về string để format nhất quán
}

/// Helper to convert a domain User model to a public UserDto
impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.usr_id,
            display_name: user.usr_display_name,
            email: user.usr_email,
            avatar_url: user.usr_avatar_url,
            role: user.usr_role,
            created_at: user.usr_created_at.map_or_else(String::new, |d| d.to_rfc3339()),
        }
    }
}