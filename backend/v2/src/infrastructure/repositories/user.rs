use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        error::RepositoryResult,
        page::Page,
        user::{
            model::{CreateUserParams, UpdateUserParams, User, UserDetail, UserMinimal, UserQuery},
            repository::IUserRepository,
        },
    },
    infrastructure::database::postgres_context::DatabasePool,
};

pub struct UserRepository {
    pool: Arc<DatabasePool>,
}

impl UserRepository {
    pub fn init(pool: Arc<DatabasePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn create_from(&self, params: &CreateUserParams) -> RepositoryResult<User> {
        todo!()
    }

    async fn get_by(&self, user_id: i32) -> RepositoryResult<UserDetail> {
        todo!()
    }

    async fn update_by(&self, params: &UpdateUserParams) -> RepositoryResult<()> {
        todo!()
    }

    async fn get_page_by(&self, query: &UserQuery) -> RepositoryResult<Page<UserMinimal>> {
        todo!()
    }

    async fn delete_by(&self, user_id: i32) -> RepositoryResult<()> {
        todo!()
    }
}
