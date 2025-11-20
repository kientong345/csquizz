use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        error::RepositoryResult,
        page::Page,
        user::{
            model::{CreateUserParams, UpdateUserParams, User, UserDetail, UserMinimal, UserQuery},
            repository::UserRepository,
        },
    },
    infrastructure::database::postgres_context::DatabasePool,
};

pub struct SqlxUserRepository {
    pool: Arc<DatabasePool>,
}

impl SqlxUserRepository {
    pub fn init(pool: Arc<DatabasePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn create(&self, params: &CreateUserParams) -> RepositoryResult<User> {
        todo!()
    }

    async fn find_by_id(&self, user_id: i32) -> RepositoryResult<UserDetail> {
        todo!()
    }

    async fn update(&self, params: &UpdateUserParams) -> RepositoryResult<()> {
        todo!()
    }

    async fn find_all(&self, query: &UserQuery) -> RepositoryResult<Page<UserMinimal>> {
        todo!()
    }

    async fn delete(&self, user_id: i32) -> RepositoryResult<()> {
        todo!()
    }
}
