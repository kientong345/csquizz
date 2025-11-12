# Tài liệu Đặc tả Kỹ thuật (Technical Specification Document)

-   **Tên dự án:** csquizz - Nền tảng Test Online về Computer Science
-   **Ngày tạo:** 2023-10-27
-   **Phiên bản:** 1.0
-   **Tình trạng:** Sơ bộ

---

## 1. Tổng quan Dự án

csquizz là một nền tảng test trực tuyến về Khoa học Máy tính. Backend của dự án cung cấp các API để quản lý người dùng, quiz, câu hỏi, kết quả nộp bài, và các tính năng tương tác xã hội như bình luận, thích.

## 2. Kiến trúc Hệ thống: Kiến trúc Phân lớp Lấy Domain làm Trung tâm (Domain-Centric Layered Architecture)

Dự án áp dụng một kiến trúc phân lớp rõ ràng, dựa trên các nguyên tắc của Clean Architecture, nhằm đảm bảo khả năng bảo trì, mở rộng và kiểm thử cao.

### 2.1. Quy tắc Phụ thuộc (The Dependency Rule)

Nguyên tắc cốt lõi là các lớp bên ngoài chỉ được phép phụ thuộc vào các lớp bên trong. Điều này tạo ra một hệ thống với lõi nghiệp vụ tách biệt và độc lập với các chi tiết kỹ thuật bên ngoài.

`Interface (Web API) -> Application (Use Cases) -> Domain (Core Business Logic)`
`Infrastructure (DB, External Services) phục vụ các Interface và Application`

### 2.2. Chi tiết các Lớp và Module

#### a. Lớp `domain` (Lõi nghiệp vụ)

-   **Mô tả:** Chứa các quy tắc nghiệp vụ cốt lõi, độc lập với bất kỳ chi tiết kỹ thuật (`framework`, `database`, `UI` nào). Đây là nơi code ít thay đổi nhất.
-   **Module con:**
    -   **`domain/models/`**: Định nghĩa các struct đại diện cho các thực thể nghiệp vụ (`User`, `Quiz`, `Question`, `Category`, `SubmissionResult`, `Answer`, `Comment`, `QuizLike`, `CommentLike`). Các enum (`UserRole`, `QuizDifficulty`, `QuestionType`) cũng được định nghĩa tại đây.
    -   **`domain/repositories/`**: Định nghĩa các trait (interface) cho các hoạt động CRUD và truy vấn dữ liệu. Ví dụ: `UserRepository`, `QuizRepository`. Lớp này chỉ định nghĩa "cần làm gì" chứ không phải "làm như thế nào".
-   **Phụ thuộc:** Không phụ thuộc vào lớp nào khác.

#### b. Lớp `application` (Dịch vụ ứng dụng)

-   **Mô tả:** Chứa các dịch vụ (services) điều phối các use case của ứng dụng. Chúng chứa logic nghiệp vụ cụ thể hơn, sử dụng các repository từ lớp `domain` để thao tác với dữ liệu.
-   **Module con:**
    -   **`application/services/`**: Chứa các service triển khai logic nghiệp vụ cụ thể (`AuthService`, `UserService`, `QuizService`, `QuestionService`, `SubmissionService`, `CategoryService`, `CommentService`, `LikeService`). Mỗi service sẽ phụ thuộc vào một hoặc nhiều `trait repository` thông qua cơ chế Dependency Injection.
    -   **`application/error.rs`**: Định nghĩa các kiểu lỗi tùy chỉnh (`ServiceError`) riêng cho lớp dịch vụ, bao gồm cả việc wrap các lỗi từ lớp `domain` (RepositoryError).
    -   **`application/app_state.rs`**: Định nghĩa struct `AppState` chứa các instance của tất cả các service và DB connection pool, được khởi tạo ở `main.rs` và truyền vào các handler của Axum.
-   **Phụ thuộc:** Chỉ phụ thuộc vào lớp `domain`.

#### c. Lớp `interface` (Cổng giao tiếp API)

-   **Mô tả:** Là điểm tiếp xúc của ứng dụng với thế giới bên ngoài (cụ thể là API HTTP). Chịu trách nhiệm nhận yêu cầu, gọi dịch vụ ứng dụng và định dạng phản hồi.
-   **Module con:**
    -   **`interface/controllers/`**: Chứa các handler (async functions) của Axum framework. Chúng parse request từ client, gọi các phương thức tương ứng trong lớp `application/services` và trả về HTTP response.
    -   **`interface/dto/`**: Chứa các Data Transfer Objects (`DTO`) được sử dụng cho request body, response body và query parameters của API. Các DTO này được thiết kế để tách biệt khỏi Domain Models, đảm bảo tính ổn định của API. Bao gồm các `Request DTO`, `Response DTO` và `Query Parameter DTO` (ví dụ: `RegisterUserDto`, `UserDto`, `ListQuizzesQuery`). Module này cũng chứa `shared_dto.rs` cho các cấu trúc chung như `PaginatedResponse` và `PaginationInfo`.
    -   **`interface/middleware/`**: Chứa các thành phần middleware của Axum, ví dụ: xử lý xác thực JWT (đảm bảo quyền truy cập, trích xuất `current_user_id`), kiểm tra vai trò người dùng (Admin, User).
    -   **`interface/routes/`**: Chứa các định nghĩa route của Axum, ánh xạ các URL path tới các controller handler tương ứng. Module này giúp tổ chức các route một cách rõ ràng và cho phép áp dụng middleware theo nhóm route.
-   **Phụ thuộc:** Phụ thuộc vào lớp `application` và `domain`.

#### d. Lớp `infrastructure` (Chi tiết kỹ thuật)

-   **Mô tả:** Chứa các chi tiết kỹ thuật, triển khai cụ thể các `trait` được định nghĩa trong lớp `domain/repositories`. Lớp này cũng quản lý kết nối với các hệ thống bên ngoài.
-   **Module con:**
    -   **`infrastructure/database/`**: Triển khai cụ thể các `trait repository` sử dụng thư viện **`sqlx`** để tương tác với PostgreSQL. Đây cũng là nơi quản lý `DbPool` (connection pool) tới database.
    -   **`infrastructure/security/`**: Chứa các dịch vụ liên quan đến bảo mật như băm mật khẩu (`password_hasher`), tạo và xác thực JWT token (`jwt_handler`).
    -   **`infrastructure/external/`**: Chứa client để giao tiếp với các dịch vụ bên ngoài, ví dụ: Google OAuth client để xác thực tài khoản Google.
-   **Phụ thuộc:** Phụ thuộc vào lớp `domain`.

### 2.3. Sơ đồ tương tác

```
+------------------+     +-----------------------+     +--------------------+     +-------------------+
|     Client       |<---->|  Interface Layer      |<--->| Application Layer  |<--->|   Domain Layer    |
| (Frontend, Mobile)|     |(Controllers, DTOs,   |     |(Services, Use Cases)|     |(Models, Repositories)|
+------------------+     |(Middleware - Axum) |     +--------------------+     +-------------------+
                                   ^                               | (uses traits)                 ^
                                   |                               v                               |
                                   +--------------------------+-----------------------------------+
                                                              |
                                                    +----------------------------+
                                                    |  Infrastructure Layer      |
                                                    | (DB, Security, External APIs)|
                                                    +----------------------------+
```

## 3. Công nghệ sử dụng

-   **Ngôn ngữ lập trình:** Rust
-   **Web Framework:** [Axum](https://docs.rs/axum/latest/axum/)
-   **Database:** PostgreSQL
-   **ORM/Database Toolkit:** [SQLx](https://github.com/launchbadge/sqlx) (hoặc Diesel) để giao tiếp bất đồng bộ với PostgreSQL.
-   **JSON Serialization/Deserialization:** [Serde](https://serde.rs/)
-   **Asynchronous Runtime:** [Tokio](https://tokio.rs/)
-   **Băm mật khẩu:** [Bcrypt](https://docs.rs/bcrypt/latest/bcrypt/) (trong `infrastructure/security/password_hasher.rs`)
-   **JSON Web Tokens (JWT):** [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/) (trong `infrastructure/security/jwt_handler.rs`)
-   **Xử lý lỗi:** [thiserror](https://github.com/dtolnay/thiserror)
-   **Trait bất đồng bộ:** [async-trait](https://docs.rs/async-trait/latest/async_trait/)
-   **Xử lý thời gian:** [Chrono](https://docs.rs/chrono/latest/chrono/)

## 4. Chi tiết Kỹ thuật

### 4.1. Cấu trúc `AppState` (Dependency Injection)

-   Một instance của `AppState` được tạo ở `main.rs`, chứa `DbPool` (pool kết nối PostgreSQL) và `Arc<dyn RepositoryTrait>` cho tất cả các repository đã triển khai trong lớp `infrastructure`.
-   Các service sẽ được khởi tạo với các `Arc<dyn RepositoryTrait>` này.
-   `AppState` sau đó được truyền làm `State` đến các controller của Axum, cho phép các handler truy cập vào các service và database.

### 4.2. Xử lý Lỗi

-   **`domain/repositories/error.rs`**: Định nghĩa `RepositoryError` để bao bọc các lỗi từ database (e.g., `sqlx::Error`) và các lỗi chung như `NotFound`. Trả về `RepositoryResult<T>`.
-   **`application/error.rs`**: Định nghĩa `ServiceError` để bao bọc `RepositoryError` và thêm các lỗi nghiệp vụ riêng (e.g., `ValidationError`, `Conflict`, `Unauthorized`, `NotFound`). Trả về `ServiceResult<T>`.
-   **`interface/controllers/`**: Các handler sẽ map `ServiceError` thành các HTTP status code và JSON response phù hợp.

### 4.3. Quản lý Database

-   Sử dụng `sqlx` để tương tác bất đồng bộ với PostgreSQL.
-   `DbPool` được khởi tạo một lần ở `main.rs` và được chia sẻ qua `AppState`.
-   Các migration được quản lý bằng `sqlx-cli` hoặc tương tự.

### 4.4. Xác thực và Phân quyền

-   **Authentication (JWT):** JTW token được tạo sau khi đăng nhập thành công. Token này được gửi trong header `Authorization` cho các request tiếp theo. `jwt_handler` (trong `infrastructure/security`) chịu trách nhiệm tạo và xác thực token.
-   **Authorization (Roles):** Middleware (trong `interface/middleware/auth_middleware.rs`) sẽ giải mã JWT, xác định `user_id` và `role` của người dùng. Controller hoặc service có thể sử dụng thông tin này để kiểm tra quyền hạn (ví dụ: chỉ Admin mới có thể tạo quiz).

### 4.5. Phân trang (Pagination)

-   Các endpoint trả về danh sách (ví dụ: `/quizzes`, `/users`, `/comments`, `/submissions`, `/questions`) sẽ hỗ trợ phân trang thông qua `Query Parameter DTO` (`page`, `limit`).
-   Response sẽ bao gồm metadada phân trang (`currentPage`, `totalPages`, `totalItems`, `limit`) trong struct `PaginatedResponse` (trong `shared_dto.rs`).

## 5. Cấu trúc Dự án chi tiết (Nhắc lại)

```
csquizz-backend/
├── src/
│   ├── application/
│   │   ├── services/         # Logic nghiệp vụ điều phối (AuthService, QuizService, ...)
│   │   ├── error.rs          # Định nghĩa ServiceError
│   │   └── app_state.rs      # AppState struct cho Dependency Injection
│   ├── domain/
│   │   ├── models/           # Định nghĩa các thực thể (User, Quiz, Question, ...)
│   │   └── repositories/     # Định nghĩa các trait (UserRepository, QuizRepository, ...)
│   ├── infrastructure/
│   │   ├── database/         # Triển khai repository dùng SQLx và PostgreSQL
│   │   ├── external/         # Client cho các dịch vụ ngoài (Google OAuth)
│   │   └── security/         # Hashing mật khẩu, JWT token
│   └── interface/
│       ├── controllers/      # Axum route handlers (auth_controller, quiz_controller, ...)
│       ├── dto/              # Data Transfer Objects (RegisterUserDto, QuizDto, ...)
│       ├── middleware/       # Axum middleware (AuthMiddleware)
│       └── routes/           # Định nghĩa các route của Axum
└── main.rs                   # Khởi tạo ứng dụng, cấu hình server Axum, DB pool
```