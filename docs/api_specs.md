# API Specifications

**Base URL:** `/api`

---

## 1. Authentication (`/auth`)

Luồng xác thực sử dụng cặp Access Token (thời gian sống ngắn) và Refresh Token (thời gian sống dài).

-   **Access Token:** Dùng để xác thực khi gọi các API cần bảo vệ. Được gửi qua header `Authorization: Bearer <access_token>`.
-   **Refresh Token:** Dùng để lấy Access Token mới. Được lưu trong một cookie `HttpOnly`, `Secure`.

### 1.1. Đăng ký người dùng mới

-   **Endpoint:** `POST /api/auth/register`
-   **Mô tả:** Tạo một tài khoản người dùng mới.
-   **Request Body:** `application/json`
    ```json
    {
        "username": "string", // Bắt buộc
        "email": "string",    // Bắt buộc, duy nhất, định dạng email
        "password": "string"  // Bắt buộc, tối thiểu 8 ký tự
    }
    ```
-   **Success Response:** `201 Created`
    ```json
    {
        "id": "number",
        "username": "string",
        "email": "string",
        "role": "string"
    }
    ```
-   **Error Responses:**
    -   `400 Bad Request`: Dữ liệu đầu vào không hợp lệ.
    -   `409 Conflict`: `username` hoặc `email` đã tồn tại.

### 1.2. Đăng nhập

-   **Endpoint:** `POST /api/auth/login`
-   **Mô tả:** Xác thực người dùng, trả về `access_token` trong body và `refresh_token` trong cookie.
-   **Request Body:** `application/json`
    ```json
    {
        "email": "string",
        "password": "string"
    }
    ```
-   **Success Response:** `200 OK`
    -   **Body:**
        ```json
        {
            "access_token": "string", // JWT Access Token (ngắn hạn)
            "user": {
                "id": "number",
                "username": "string",
                "role": "string"
            }
        }
        ```
    -   **Headers:**
        -   `Set-Cookie`: `refresh_token=...; HttpOnly; Secure; Path=/api/auth`

-   **Error Responses:**
    -   `401 Unauthorized`: Sai `email` hoặc `password`.

### 1.3. Làm mới Access Token

-   **Endpoint:** `POST /api/auth/refresh`
-   **Mô tả:** Lấy một `access_token` mới khi cái cũ hết hạn.
-   **Authentication:** Trình duyệt tự động gửi `refresh_token` qua cookie.
-   **Request Body:** (empty)
-   **Success Response:** `200 OK`
    ```json
    {
        "access_token": "string" // Access Token mới
    }
    ```
-   **Error Responses:**
    -   `401 Unauthorized`: `refresh_token` không hợp lệ hoặc đã hết hạn.

### 1.4. Đăng xuất

-   **Endpoint:** `POST /api/auth/logout`
-   **Mô tả:** Vô hiệu hóa `refresh_token` và xóa cookie khỏi trình duyệt.
-   **Authentication:** Trình duyệt tự động gửi `refresh_token` qua cookie.
-   **Success Response:** `204 No Content`
    -   **Headers:**
        -   `Set-Cookie`: `refresh_token=; HttpOnly; Secure; Path=/api/auth; Max-Age=0` (xóa cookie)
-   **Error Responses:**
    -   `401 Unauthorized`: `refresh_token` không hợp lệ.

### 1.5. Lấy thông tin người dùng hiện tại

-   **Endpoint:** `GET /api/auth/me`
-   **Mô tả:** Lấy thông tin của người dùng đang đăng nhập dựa trên `access_token`.
-   **Authentication:** Yêu cầu `Bearer <access_token>` trong header `Authorization`.
-   **Success Response:** `200 OK`
    ```json
    {
        "id": "number",
        "username": "string",
        "email": "string",
        "role": "string",
        "created_at": "string"
    }
    ```
-   **Error Responses:**
    -   `401 Unauthorized`: `access_token` không hợp lệ hoặc hết hạn.

---

## 2. Quizzes (`/quizzes`)

### 2.1. Lấy danh sách các quiz

-   **Endpoint:** `GET /api/quizzes`
-   **Mô tả:** Lấy danh sách các bài quiz có sẵn, hỗ trợ tìm kiếm và lọc.
-   **Query Parameters:**
    -   `title_pattern`: `string` (optional)
    -   `category`: `string` (optional)
    -   `difficulty`: `string` (optional)
    -   `page`: `number`
    -   `size`: `number`
-   **Success Response:** `200 OK` (Nội dung không đổi)

### 2.2. Lấy thông tin 1 quiz

-   **Endpoint:** `GET /api/quizzes/{id}`
-   **Success Response:** `200 OK` (Nội dung không đổi)

### 2.3. Lấy chi tiết một bài quiz (để làm bài)

-   **Endpoint:** `GET /api/quizzes/{id}/questions`
-   **Success Response:** `200 OK` (Nội dung không đổi)

### 2.4. Nộp bài và chấm điểm

-   **Endpoint:** `POST /api/quizzes/{id}/submit`
-   **Authentication:** Tùy chọn. Nếu có `Bearer <access_token>`, kết quả sẽ được lưu vào lịch sử của người dùng.
-   **Success Response:** `200 OK` (Nội dung không đổi)

---

## 3. Categories (`/categories`)

### 3.1. Lấy danh sách category

-   **Endpoint:** `GET /api/categories`
-   **Success Response:** `200 OK` (Nội dung không đổi)

---

## 4. User (`/user`)

### 4.1. Lấy lịch sử làm bài

-   **Endpoint:** `GET /api/user/results`
-   **Mô tả:** Lấy danh sách các kết quả quiz của người dùng đang đăng nhập.
-   **Authentication:** Yêu cầu `Bearer <access_token>`.
-   **Success Response:** `200 OK` (Nội dung không đổi)
-   **Error Responses:**
    -   `401 Unauthorized`: `access_token` không hợp lệ.

---

## 5. Admin (`/admin`)

Tất cả các endpoint trong mục này đều yêu cầu quyền `admin`.

-   **Authentication:** Yêu cầu `Bearer <access_token>` với `role` là `admin`.
-   `POST /api/admin/quizzes`: Tạo một quiz mới.
-   `PUT /api/admin/quizzes/{id}`: Cập nhật thông tin một quiz.
-   `DELETE /api/admin/quizzes/{id}`: Xóa một quiz.
-   `POST /api/admin/quizzes/{quizId}/questions`: Thêm một câu hỏi mới vào quiz.
-   `PUT /api/admin/questions/{questionId}`: Cập nhật một câu hỏi.
-   `DELETE /api/admin/questions/{questionId}`: Xóa một câu hỏi.