use serde::Serialize;

/// DTO chung cho các response có phân trang.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

/// Thông tin phân trang.
#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    #[serde(rename = "currentPage")]
    pub current_page: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    pub limit: i64,
}
