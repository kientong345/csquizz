use serde::Serialize;

use crate::models::user::{UserFullDetail, UserMinimal, UserPubInfo, UserRole};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMinimalDto {
    pub id: i32,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPubInfoDto {
    pub id: i32,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub quiz_created_count: i64,
    pub quiz_completed_count: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFullDetailDto {
    pub pub_info: UserPubInfoDto,
    pub email: String,
    pub password_hash: Option<String>,
    pub google_id: Option<String>,
}

impl Into<UserMinimalDto> for UserMinimal {
    fn into(self) -> UserMinimalDto {
        UserMinimalDto {
            id: self.id,
            display_name: self.display_name,
            avatar_url: self.avatar_url,
        }
    }
}

impl Into<UserPubInfoDto> for UserPubInfo {
    fn into(self) -> UserPubInfoDto {
        let role = match self.role {
            UserRole::Admin => String::from("admin"),
            UserRole::User => String::from("user"),
        };

        UserPubInfoDto {
            id: self.id,
            display_name: self.display_name,
            avatar_url: self.avatar_url,
            role,
            quiz_created_count: self.quiz_created_count,
            quiz_completed_count: self.quiz_completed_count,
        }
    }
}

impl Into<UserFullDetailDto> for UserFullDetail {
    fn into(self) -> UserFullDetailDto {
        UserFullDetailDto {
            pub_info: self.pub_info.into(),
            email: self.email,
            password_hash: self.password_hash,
            google_id: self.google_id,
        }
    }
}
