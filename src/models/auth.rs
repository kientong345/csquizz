use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Registration {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl Registration {
    pub fn is_valid(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
pub struct Logination {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthPayload {
    pub google_id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub enum SignupMethod {
    WithPassword(Registration),
    OAuth(OAuthPayload),
}
