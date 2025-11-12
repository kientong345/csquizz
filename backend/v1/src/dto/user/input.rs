use serde::Deserialize;

use crate::models::user::{OrderType, paginate::UserQuery, post::PostUser};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserQueryDto {
    pub order_by: Option<String>,
    pub page: i64,
    pub size: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostUserDto {
    pub google_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<String>, // "user" || "admin"
}

impl From<UserQueryDto> for UserQuery {
    fn from(value: UserQueryDto) -> Self {
        let order_by = if let Some(order) = value.order_by {
            match order.as_ref() {
                "most-created" => OrderType::MostCreated,
                "most-solve" => OrderType::MostSolved,
                _ => OrderType::MostCreated,
            }
        } else {
            OrderType::MostCreated
        };

        Self {
            order_by,
            page: value.page,
            size: value.size,
        }
    }
}

impl From<PostUserDto> for PostUser {
    fn from(value: PostUserDto) -> Self {
        Self {
            google_id: value.google_id,
            display_name: value.display_name,
            email: value.email,
            password_hash: value.password_hash,
            avatar_url: value.avatar_url,
            role: value.role,
        }
    }
}
