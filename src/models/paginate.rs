use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, prelude::FromRow, PgConnection, Postgres};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub total_pages: i64,
}

#[allow(async_fn_in_trait)]
pub trait Paginate<Q>: Serialize + Sized {
    async fn page(query: &Q, connection: &mut PgConnection) -> Result<Page<Self>, sqlx::Error>;
}
