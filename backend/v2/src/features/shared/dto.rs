use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::page::Page;

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageDto<T> {
    #[schema(example = "items")]
    pub items: Vec<T>,

    #[schema(example = 1)]
    pub current_page: i64,

    #[schema(example = 10)]
    pub total_pages: i64,

    #[schema(example = 100)]
    pub total_items: i64,

    #[schema(example = 10)]
    pub limit: i64,
}

impl<T> From<Page<T>> for PageDto<T> {
    fn from(value: Page<T>) -> Self {
        PageDto {
            items: value.items,
            current_page: value.current_page,
            total_pages: value.total_pages,
            total_items: value.total_items,
            limit: value.limit,
        }
    }
}
