use serde::Serialize;
use sqlx::PgConnection;

use crate::models::error::ModelError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub total_pages: i64,
}

impl<T> Page<T> {
    pub fn build_from(items: Vec<T>, total_items: i64, page_size: i32) -> Self {
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

    pub fn map_into<U: From<T>>(self) -> Page<U> {
        Page::<U> {
            items: self.items.into_iter().map(|e| U::from(e)).collect(),
            total_items: self.total_items,
            total_pages: self.total_pages,
        }
    }

    pub fn try_map_into<U: TryFrom<T, Error = ModelError>>(self) -> Result<Page<U>, ModelError> {
        let items = self
            .items
            .into_iter()
            .map(U::try_from)
            .collect::<Result<Vec<U>, ModelError>>()?;

        Ok(Page {
            items,
            total_items: self.total_items,
            total_pages: self.total_pages,
        })
    }
}

#[allow(async_fn_in_trait)]
pub trait Paginate<Q>: Serialize + Sized {
    async fn page(params: &Q, connection: &mut PgConnection) -> Result<Page<Self>, ModelError>;
}
