use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, prelude::FromRow, Postgres};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub total_page: i32,
    pub page: i32,
    pub size: i32,
}

#[allow(async_fn_in_trait)]
pub trait Paginate<Q>: Serialize + Sized {
    async fn page(
        query: &Q,
        connection: &PoolConnection<Postgres>,
    ) -> Result<Page<Self>, sqlx::Error>;
}
