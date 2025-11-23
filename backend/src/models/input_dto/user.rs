use serde::Deserialize;

use crate::models::user::UserUpdateParams;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserUpdateParamsDto {
    pub display_name: Option<String>,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
}

impl UserUpdateParamsDto {
    pub fn bind(self, id: i32) -> UserUpdateParams {
        UserUpdateParams {
            id,
            display_name: self.display_name,
            password_hash: self.password_hash,
            avatar_url: self.avatar_url,
        }
    }
}
