use std::sync::Arc;

use crate::{
    config::auth::AuthConfig,
    domain::user::repository::UserRepository,
    features::auth::{
        dto::{GoogleLoginDto, LoginDto, RegisterUserDto},
        error::AuthResult,
    },
};

pub struct AuthService {
    repository: Arc<dyn UserRepository>,
    config: AuthConfig,
}

impl AuthService {
    pub fn build_from(repository: Arc<dyn UserRepository>, config: AuthConfig) -> Self {
        Self { config, repository }
    }

    pub async fn register(&self, dto: RegisterUserDto) -> AuthResult<()> {
        // 1. Check if user already exists
        // if self.user_repo.find_by_email(&dto.email).await?.is_some() {
        //     return Err(ServiceError::Conflict("User with this email already exists".to_string()));
        // }

        // 2. Hash the password
        // let hashed_password = self.hasher.hash(&dto.password)?;

        // 3. Create the user
        // self.user_repo.create(&dto.display_name, &dto.email, &hashed_password).await?;

        println!("Attempting to register user: {}", dto.email); // Placeholder
        Ok(())
    }

    pub async fn login(&self, dto: LoginDto) -> AuthResult<String> {
        // 1. Find user by email
        // let user = self.user_repo.find_by_email(&dto.email).await?
        //     .ok_or_else(|| ServiceError::Unauthorized("Invalid credentials".to_string()))?;

        // 2. Verify password
        // let password_hash = user.usr_password_hash.as_ref()
        //     .ok_or_else(|| ServiceError::Unauthorized("User is configured for social login".to_string()))?;
        // if !self.hasher.verify(password_hash, &dto.password)? {
        //     return Err(ServiceError::Unauthorized("Invalid credentials".to_string()));
        // }

        // 3. Generate JWT
        // let token = self.jwt_handler.generate(user.usr_id, user.usr_role)?;

        println!("Attempting to log in user: {}", dto.email); // Placeholder
        Ok("placeholder_jwt_token".to_string())
    }

    pub async fn google_login(&self, _dto: GoogleLoginDto) -> AuthResult<String> {
        // 1. Verify Google token and get user info (using a client in infrastructure/external)
        // 2. Check if user exists in DB (find_by_google_id or find_by_email)
        // 3. If not, create a new user.
        // 4. If exists, update info if necessary.
        // 5. Generate JWT for the user.

        println!("Attempting Google login"); // Placeholder
        Ok("placeholder_jwt_token_from_google".to_string())
    }
}
