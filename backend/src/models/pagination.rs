use serde::Serialize;
use sqlx::PgConnection;

use crate::models::error::ModelError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub total_pages: i64,
}

impl<T> Page<T> {
    pub fn build_from(items: Vec<T>, total_items: i64, page_size: i64) -> Self {
        let total_pages = if page_size > 0 {
            (total_items as f64 / page_size as f64).ceil() as i64
        } else {
            0
        };

        Self {
            items,
            total_items,
            total_pages,
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait Paginate<Q>: Serialize + Sized {
    async fn page(query: &Q, connection: &mut PgConnection) -> Result<Page<Self>, ModelError>;
}
