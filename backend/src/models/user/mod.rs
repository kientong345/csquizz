use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::error::ModelError;

pub mod create;
pub mod delete;
pub mod get;
pub mod paginate;
pub mod update;

#[derive(Debug, Clone, Serialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    User,
    Admin,
}

impl ToString for UserRole {
    fn to_string(&self) -> String {
        match self {
            UserRole::Admin => String::from("admin"),
            UserRole::User => String::from("user"),
        }
    }
}

impl FromStr for UserRole {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(UserRole::User),
            "admin" => Ok(UserRole::Admin),
            _ => Err(ModelError::BadPost(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UserSortField {
    QuizCompletedCount,
    QuizCreatedCount,
    FollowerCount,
}

impl FromStr for UserSortField {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "quiz-completed-count" => Ok(UserSortField::QuizCompletedCount),
            "quiz-created-count" => Ok(UserSortField::QuizCreatedCount),
            "follower-count" => Ok(UserSortField::FollowerCount),
            _ => Err(ModelError::BadPost(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseUser {
    pub id: i32,
    pub google_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserMinimal {
    pub id: i32,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserPublicDetail {
    pub id: i32,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub created_at: Option<String>,
    pub quiz_completed_count: i64,
    pub quiz_created_count: i64,
    pub follower_count: i64,
}

impl From<UserFullDetail> for UserPublicDetail {
    fn from(value: UserFullDetail) -> Self {
        Self {
            id: value.id,
            display_name: value.display_name,
            avatar_url: value.avatar_url,
            created_at: value.created_at,
            quiz_completed_count: value.quiz_completed_count,
            quiz_created_count: value.quiz_created_count,
            follower_count: value.follower_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserFullDetail {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub created_at: Option<String>,
    pub quiz_completed_count: i64,
    pub quiz_created_count: i64,
    pub follower_count: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserCreateParams {
    pub google_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserUpdateParams {
    pub id: i32,
    pub display_name: Option<String>,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPaginateParams {
    pub name_pattern: String,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: String,
}
