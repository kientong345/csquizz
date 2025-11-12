use serde::Serialize;

use crate::models::pagination::Page;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDto<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    pub total_pages: i64,
}

impl<T, U: Into<T>> Into<PageDto<T>> for Page<U> {
    fn into(self) -> PageDto<T> {
        let mut items = Vec::new();
        for item in self.items {
            items.push(item.into());
        }
        PageDto::<T> {
            items,
            total_items: self.total_items,
            total_pages: self.total_pages,
        }
    }
}
