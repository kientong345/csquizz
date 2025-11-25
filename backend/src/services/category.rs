use serde_json::{Value, json};
use sqlx::PgConnection;

use crate::{
    models::{
        category::{Category, CategoryCreateParams, CategoryPaginateParams},
        input_dto::category::CategoryUpdateParamsDto,
        pagination::Paginate,
    },
    services::error::ServiceError,
};

#[derive(Clone)]
pub struct CategoryService;

impl CategoryService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_page(
        &self,
        conn: &mut PgConnection,
        query: &CategoryPaginateParams,
    ) -> Result<Value, ServiceError> {
        let page = Category::page(query, conn).await?;
        Ok(json!(page))
    }

    pub async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: i32,
    ) -> Result<Value, ServiceError> {
        let categories = Category::get_by_id(id, conn).await?;
        Ok(json!(categories))
    }

    pub async fn find_all(&self, conn: &mut PgConnection) -> Result<Value, ServiceError> {
        let categories = Category::list_all(conn).await?;
        Ok(json!(categories))
    }

    pub async fn create(
        &self,
        conn: &mut PgConnection,
        payload: &CategoryCreateParams,
    ) -> Result<(), ServiceError> {
        Category::create_from(payload, conn).await?;
        Ok(())
    }

    pub async fn delete(&self, conn: &mut PgConnection, id: i32) -> Result<(), ServiceError> {
        Category::delete_by(id, conn).await?;
        Ok(())
    }

    pub async fn update(
        &self,
        conn: &mut PgConnection,
        id: i32,
        payload: &CategoryUpdateParamsDto,
    ) -> Result<(), ServiceError> {
        let params = payload.clone().bind(id);
        Category::update_by(&params, conn).await?;
        Ok(())
    }
}
