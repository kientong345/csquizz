# Tài liệu Đặc tả Kỹ thuật (Technical Specification Document)

-   **Tên dự án:** csquizz - Nền tảng Test Online về Computer Science
-   **Ngày tạo:** 2023-10-27
-   **Phiên bản:** 1.1 (Cập nhật ngày 2025-11-13)
-   **Tình trạng:** Đang phát triển

---

## 1. Tổng quan Dự án

csquizz là một nền tảng test trực tuyến về Khoa học Máy tính. Backend của dự án cung cấp các API để quản lý người dùng, quiz, câu hỏi, kết quả nộp bài, và các tính năng tương tác xã hội như bình luận, thích.

## 2. Kiến trúc Hệ thống: Hybrid giữa Lát cắt dọc (VSA) và Thiết kế Hướng Miền (DDD)

Dự án áp dụng một kiến trúc hybrid tiên tiến, kết hợp sức mạnh của hai trường phái thiết kế phần mềm hiện đại:

1.  **Thiết kế Hướng Miền (Domain-Driven Design - DDD):** Được sử dụng để xây dựng một lõi nghiệp vụ (domain core) vững chắc, độc lập và dễ hiểu. DDD giúp mô hình hóa các quy tắc nghiệp vụ phức tạp một cách rõ ràng.
2.  **Kiến trúc Lát cắt dọc (Vertical Slice Architecture - VSA):** Được sử dụng để tổ chức mã nguồn theo từng tính năng (feature). Mỗi "lát cắt" là một module độc lập chứa tất cả logic cần thiết cho một use case cụ thể, từ API endpoint đến logic ứng dụng.

Sự kết hợp này cho phép dự án vừa có một nền tảng nghiệp vụ ổn định, vừa có cấu trúc code dễ bảo trì và mở rộng theo từng tính năng.

### 2.1. Quy tắc Phụ thuộc (The Dependency Rule)

Quy tắc cốt lõi vẫn được tuân thủ: các thành phần bên ngoài chỉ được phụ thuộc vào các thành phần bên trong.

`API (Routes) -> Features (Slices) -> Domain (Core Business Logic)`
`Infrastructure (DB, External Services) -> Domain (implements Repository Traits)`

### 2.2. Chi tiết các Lớp và Thành phần

#### a. Lớp `domain` (Lõi DDD)

-   **Mô tả:** Là trái tim của ứng dụng, chứa các quy tắc nghiệp vụ cốt lõi và hoàn toàn độc lập với các chi tiết kỹ thuật. Đây là lớp ổn định nhất, được xây dựng theo các nguyên tắc của DDD.
-   **Thành phần chính:**
    -   **`domain/{entity}/model.rs`**: Định nghĩa các **Entities** (`User`, `Quiz`, `Comment`...) - các đối tượng nghiệp vụ có định danh và vòng đời.
    -   **`domain/{entity}/repository.rs`**: Định nghĩa các **Repository Traits** (interfaces). Đây là "hợp đồng" quy định các phương thức truy xuất dữ liệu cho các entity, giúp tách biệt hoàn toàn lớp domain khỏi cách lưu trữ dữ liệu cụ thể.

#### b. Lớp `features` (Các Lát cắt dọc)

-   **Mô tả:** Đây là nơi triển khai VSA. Mỗi thư mục con trong `features` là một "lát cắt dọc" tương ứng với một nhóm tính năng nghiệp vụ (ví dụ: `features/quiz`, `features/user`).
-   **Cấu trúc một lát cắt:** Mỗi lát cắt chứa logic ứng dụng và giao diện cho tính năng đó, giúp tăng tính gắn kết (cohesion) và giảm sự phụ thuộc (coupling) giữa các tính năng.
    -   **`controller.rs`**: Chứa các handler của Axum (API endpoints). Đây là điểm bắt đầu của một use case.
    -   **`service.rs`**: Đóng vai trò **Application Service** trong DDD, điều phối luồng thực thi của use case, gọi các phương pháp từ repository (thông qua traits) và xử lý logic ứng dụng.
    -   **`dto.rs`**: Chứa các Data Transfer Objects (DTOs) được sử dụng để giao tiếp với client, đảm bảo dữ liệu của lớp `domain` không bị lộ ra ngoài.
    -   **`error.rs`**: Định nghĩa các lỗi cụ thể cho tính năng đó.

#### c. Lớp `api` (Gateway và Middleware)

-   **Mô tả:** Đóng vai trò là lớp "gateway" mỏng, chịu trách nhiệm chính cho việc định tuyến và các vấn đề xuyên suốt (cross-cutting concerns).
-   **Thành phần chính:**
    -   **`api/routes/`**: Định nghĩa và kết nối tất cả các router từ các `controller` trong lớp `features` vào ứng dụng Axum chính.
    -   **`api/middleware/`**: Chứa các middleware dùng chung như xác thực token (JWT), logging, CORS...

#### d. Lớp `infrastructure` (Chi tiết Kỹ thuật)

-   **Mô tả:** Chứa các chi tiết kỹ thuật và triển khai cụ thể cho các "hợp đồng" đã được định nghĩa trong lớp `domain`.
-   **Thành phần chính:**
    -   **`infrastructure/repositories/`**: Chứa các file triển khai repository cụ thể (ví dụ: `PostgresQuizRepository`). Đây là nơi chứa logic **SQL** thực tế để giao tiếp với database PostgreSQL.
    -   **`infrastructure/security/`**: Chứa các dịch vụ liên quan đến bảo mật như băm mật khẩu.
    -   **`infrastructure/external/`**: Chứa client để giao tiếp với các dịch vụ bên ngoài (ví dụ: Google OAuth).

### 2.3. Sơ đồ tương tác

Sơ đồ này minh họa luồng xử lý một request trong kiến trúc hybrid.

```
           +-----------------------------------------------------------------+
           |                           features/quiz                         |
           |                                                                 |
(Request)  |  +----------------+   (Call)   +---------------+                |
---------> |  | controller.rs  |----------->|  service.rs   |                |
           |  | (API Endpoint) |            | (Application  |                |
           |  +----------------+            |    Logic)     |                |
           |         ^                      +---------------+                |
           |         | (Returns DTO)               | (Uses Trait)           |
           +---------------------------------------|-------------------------+
                                                   |
                 +---------------------------------|-----------------------------------+
                 |                                 v                                   |
           +----------------+            +--------------------+            +--------------------+
           |  domain/quiz/  |  (Defines) | domain/quiz/       | (Implements) | infrastructure/    |
           |  model.rs      |<-----------| repository.rs      |<-------------| repositories/      |
           |  (Entity)      |            | (Repository Trait) |            | quiz.rs            |
           +----------------+            +--------------------+            | (SQL Logic)        |
                                                                           +--------------------+
```

## 3. Công nghệ sử dụng

-   **Ngôn ngữ lập trình:** Rust
-   **Web Framework:** [Axum](https://docs.rs/axum/latest/axum/)
-   **Database:** PostgreSQL
-   **Database Toolkit:** [SQLx](https://github.com/launchbadge/sqlx)
-   **JSON:** [Serde](https://serde.rs/)
-   **Asynchronous Runtime:** [Tokio](https://tokio.rs/)
-   **Xử lý lỗi:** [thiserror](https://github.com/dtolnay/thiserror)
-   **Trait bất đồng bộ:** [async-trait](https://docs.rs/async-trait/latest/async_trait/)
-   **Cấu hình:** [dotenvy](https://github.com/allan2/dotenvy)

## 4. Chi tiết Kỹ thuật

### 4.1. Dependency Injection trong `AppState`

-   `AppState` được khởi tạo ở `main.rs`.
-   Các triển khai repository cụ thể từ lớp `infrastructure` (ví dụ: `PostgresQuizRepository`) được tạo và bọc trong `Arc<dyn QuizRepository>`.
-   Các `Arc` này được inject vào các service.
-   Các service lại được bọc trong `Arc` và đưa vào `AppState`, sau đó được Axum quản lý và cung cấp cho các controller.

### 4.2. Xử lý Lỗi

-   **`RepositoryError`**: Lỗi từ lớp `infrastructure` (ví dụ: `sqlx::Error`) được wrap thành `RepositoryError` trong lớp `domain`.
-   **`ServiceError`**: Lớp `application` xử lý `RepositoryError` và các lỗi nghiệp vụ khác (validation, conflict...), chuyển đổi chúng thành `ServiceError`.
-   **`impl IntoResponse for ServiceError`**: Lớp `interface` (trong controller) định nghĩa cách chuyển đổi `ServiceError` thành HTTP status code và JSON response phù hợp, giúp giữ cho logic trong handler gọn gàng.

### 4.3. Phân trang (Pagination)

-   Các phương thức repository trả về danh sách sẽ trả về một tuple `(Vec<Item>, u32)` chứa dữ liệu trang hiện tại và tổng số mục.
-   Service sẽ sử dụng thông tin này để tính toán và tạo `PaginationInfo`.
-   Controller trả về `PaginatedResponse<T>` chứa cả dữ liệu và thông tin phân trang.

### 4.4. Luồng triển khai một tính năng (Ví dụ: Sắp xếp)

-   **Interface (`dto`):** Tham số `sort_by: Option<String>` được thêm vào `ListQuizzesQuery` DTO.
-   **Application (`services`):** Service nhận chuỗi `sort_by` thô. Nó chịu trách nhiệm **xác thực (validate)** chuỗi này và chuyển nó thành một `enum` an toàn về kiểu (type-safe) của lớp Domain (ví dụ: `QuizSortField::LikeCount`). Nếu chuỗi không hợp lệ, service sẽ trả về lỗi `BadRequest`.
-   **Domain (`repositories`):** Struct `ListQuizzesParams` được cập nhật để nhận `enum` `QuizSortField` từ service.
-   **Infrastructure (`database`):** Triển khai repository sử dụng `QueryBuilder` của `sqlx`. Nó `match` giá trị enum `QuizSortField` để **xây dựng động (dynamically construct)** mệnh đề `ORDER BY` một cách an toàn, tránh nguy cơ SQL Injection.
