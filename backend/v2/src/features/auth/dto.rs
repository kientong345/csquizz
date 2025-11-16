use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
pub struct LoginResponseDto {
    #[serde(rename = "tokenType")]
    pub token_type: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
}
