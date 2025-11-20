use std::sync::Arc;

use crate::domain::user::repository::UserRepository;

#[derive(Clone)]
pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn build_from(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    // pub async fn get_user_profile(&self, user_id: i32) -> ServiceResult<User> {
    //     // self.user_repo.find_by_id(user_id).await?
    //     //     .ok_or_else(|| ServiceError::NotFound(format!("User with ID {} not found", user_id)))
    //     println!("Getting user profile for ID: {}", user_id); // Placeholder
    //     Err(ServiceError::NotFound("User not found".to_string()))
    // }

    // pub async fn update_user_profile(
    //     &self,
    //     user_id: i32,
    //     dto: UpdateUserProfileDto,
    // ) -> ServiceResult<()> {
    //     // let mut user = self.user_repo.find_by_id(user_id).await?
    //     //     .ok_or_else(|| ServiceError::NotFound(format!("User with ID {} not found", user_id)))?;
    //     // if let Some(display_name) = dto.display_name {
    //     //     user.usr_display_name = display_name;
    //     // }
    //     // if let Some(avatar_url) = dto.avatar_url {
    //     //     user.usr_avatar_url = Some(avatar_url);
    //     // }
    //     // self.user_repo.update(user).await?;
    //     println!("Updating user profile for ID: {}", user_id); // Placeholder
    //     Ok(())
    // }

    // pub async fn list_users(&self, query: ListUsersQuery) -> ServiceResult<(Vec<User>, i64, i64)> {
    //     // let users = self.user_repo.list(query).await?;
    //     // let total_items = self.user_repo.count().await?;
    //     // let total_pages = (total_items as f64 / query.limit.unwrap_or(20) as f64).ceil() as i64;
    //     println!("Listing users with query: {:?}", query); // Placeholder
    //     Ok((vec![], 0, 0))
    // }

    // pub async fn update_user_role(
    //     &self,
    //     user_id: i32,
    //     dto: UpdateUserRoleDto,
    // ) -> ServiceResult<()> {
    //     // self.user_repo.update_role(user_id, dto.role).await?;
    //     println!("Updating role for user ID: {} to {:?}", user_id, dto.role); // Placeholder
    //     Ok(())
    // }
}
