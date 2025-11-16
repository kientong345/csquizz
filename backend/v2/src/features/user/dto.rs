use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::user::model::{
    CreateUserParams, UpdateUserParams, UserDetail, UserMinimal, UserQuery,
};

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserMinimalDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "John Doe")]
    pub display_name: String,

    #[schema(example = "https://example.com/avatar.png")]
    pub avatar_url: Option<String>,

    #[schema(example = 10)]
    pub quiz_completed_count: i64,

    #[schema(example = 5)]
    pub quiz_created_count: i64,
}

impl From<UserMinimal> for UserMinimalDto {
    fn from(user: UserMinimal) -> Self {
        Self {
            id: user.id,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            quiz_completed_count: user.quiz_completed_count,
            quiz_created_count: user.quiz_created_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PrivateUserDetailDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "google-123")]
    pub google_id: Option<String>,

    #[schema(example = "John Doe")]
    pub display_name: String,

    #[schema(example = "john.doe@example.com")]
    pub email: String,

    #[schema(example = "https://example.com/avatar.png")]
    pub avatar_url: Option<String>,

    #[schema(example = "user")]
    pub role: String,

    #[schema(example = "2023-01-01T12:00:00Z")]
    pub created_at: Option<String>,

    #[schema(example = 10)]
    pub quiz_completed_count: i64,

    #[schema(example = 5)]
    pub quiz_created_count: i64,
}

impl From<UserDetail> for PrivateUserDetailDto {
    fn from(value: UserDetail) -> Self {
        Self {
            id: value.user.id,
            google_id: value.user.google_id,
            display_name: value.user.display_name,
            email: value.user.email,
            avatar_url: value.user.avatar_url,
            role: match value.user.role {
                crate::domain::user::model::UserRole::User => "user".to_string(),
                crate::domain::user::model::UserRole::Admin => "admin".to_string(),
            },
            created_at: value.user.created_at.map(|dt| dt.to_rfc3339()),
            quiz_completed_count: value.quiz_completed_count,
            quiz_created_count: value.quiz_created_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserDetailDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "John Doe")]
    pub display_name: String,

    #[schema(example = "https://example.com/avatar.png")]
    pub avatar_url: Option<String>,

    #[schema(example = "2023-01-01T12:00:00Z")]
    pub created_at: Option<String>,

    #[schema(example = 10)]
    pub quiz_completed_count: i64,

    #[schema(example = 5)]
    pub quiz_created_count: i64,
}

impl From<UserDetail> for PublicUserDetailDto {
    fn from(value: UserDetail) -> Self {
        Self {
            id: value.user.id,
            display_name: value.user.display_name,
            avatar_url: value.user.avatar_url,
            created_at: value.user.created_at.map(|dt| dt.to_rfc3339()),
            quiz_completed_count: value.quiz_completed_count,
            quiz_created_count: value.quiz_created_count,
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct CreateUserParamsDto {
    #[schema(example = "google-123")]
    pub google_id: Option<String>,

    #[schema(example = "John Doe")]
    pub display_name: String,

    #[schema(example = "john.doe@example.com")]
    pub email: String,

    #[schema(example = "password123")]
    pub password: Option<String>,

    #[schema(example = "https://example.com/avatar.png")]
    pub avatar_url: Option<String>,

    #[schema(example = "user")]
    pub role: String,
}

impl From<CreateUserParamsDto> for CreateUserParams {
    fn from(value: CreateUserParamsDto) -> Self {
        Self {
            google_id: value.google_id,
            display_name: value.display_name,
            email: value.email,
            password_hash: value.password.map(|p| p), // In real code, hash the password
            avatar_url: value.avatar_url,
            role: match value.role.as_str() {
                "admin" => crate::domain::user::model::UserRole::Admin,
                _ => crate::domain::user::model::UserRole::User,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct UpdateUserParamsDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "John Doe")]
    pub display_name: Option<String>,

    #[schema(example = "password123")]
    pub password: Option<String>,

    #[schema(example = "https://example.com/avatar.png")]
    pub avatar_url: Option<String>,
}

impl From<UpdateUserParamsDto> for UpdateUserParams {
    fn from(value: UpdateUserParamsDto) -> Self {
        Self {
            id: value.id,
            display_name: value.display_name,
            password_hash: value.password.map(|p| p), // In real code, hash the password
            avatar_url: value.avatar_url,
            role: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserQueryDto {
    #[schema(example = "john")]
    pub name_pattern: String,

    #[schema(example = 1)]
    pub page: i32,

    #[schema(example = 20)]
    pub limit: i32,

    #[schema(example = "quiz_completed_count")]
    pub sort_by: String,
}

impl From<UserQueryDto> for UserQuery {
    fn from(value: UserQueryDto) -> Self {
        Self {
            name_pattern: value.name_pattern,
            page: value.page,
            limit: value.limit,
            sort_by: match value.sort_by.as_str() {
                "quiz_created_count" => crate::domain::user::model::UserSortField::QuizCreatedCount,
                _ => crate::domain::user::model::UserSortField::QuizCompletedCount,
            },
        }
    }
}
