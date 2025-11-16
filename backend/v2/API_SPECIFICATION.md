# Đặc tả API (API Specification) cho Dự án csquizz

-   **Phiên bản:** 1.1
-   **Định dạng:** JSON

## 1. Nguyên tắc thiết kế

-   **RESTful & No-nesting:** API được thiết kế theo các nguyên tắc REST. Các endpoint được giữ phẳng.
-   **Stateless & JWT:** Mọi request cần xác thực phải chứa `Bearer Token` (JWT) trong header `Authorization`.
-   **Summary vs. Detail DTOs:**
    -   Các endpoint trả về danh sách (List View) sẽ sử dụng DTO dạng **Summary** (tóm tắt), chứa các thông tin cần thiết để hiển thị trên danh sách, bao gồm cả các dữ liệu tổng hợp (như `question_count`, `like_count`).
    -   Các endpoint trả về chi tiết một đối tượng (Detail View) sẽ sử dụng DTO dạng **Detail**, chứa đầy đủ thông tin của đối tượng đó.

---

## 2. Endpoint chi tiết

### FR1: Quản lý Người dùng & Xác thực

#### `POST /register`

-   **Mô tả:** Đăng ký một tài khoản người dùng mới.
-   **Body:**
    ```json
    {
      "display_name": "John Doe",
      "email": "john.doe@example.com",
      "password": "strongpassword123"
    }
    ```
-   **Response (201 Created):**
    ```json
    {
      "status": "success",
      "message": "User registered successfully."
    }
    ```

#### `POST /login`

-   **Mô tả:** Đăng nhập và nhận về một JWT.
-   **Body:**
    ```json
    {
      "email": "john.doe@example.com",
      "password": "strongpassword123"
    }
    ```
-   **Response (200 OK):**
    ```json
    {
      "token_type": "Bearer",
      "access_token": "your-jwt-token"
    }
    ```

#### `POST /login/google`

-   **Mô tả:** Xử lý callback từ Google OAuth để đăng nhập hoặc đăng ký.
-   **Body:**
    ```json
    {
      "google_token": "token-from-google-client"
    }
    ```
-   **Response (200 OK):** (Tương tự `/login`)

#### `GET /users/me`

-   **Mô tả:** Lấy thông tin hồ sơ của người dùng hiện tại.
-   **Xác thực:** Yêu cầu JWT.
-   **Response (200 OK):**
    ```json
    {
      "id": 1,
      "display_name": "John Doe",
      "email": "john.doe@example.com",
      "avatar_url": "url-to-avatar",
      "role": "user",
      "created_at": "2023-10-27T10:00:00Z"
    }
    ```

#### `PATCH /users/me`

-   **Mô tả:** Cập nhật thông tin hồ sơ của người dùng hiện tại.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:** (Các trường là tùy chọn)
    ```json
    {
      "display_name": "Johnathan Doe",
      "avatar_url": "new-url-to-avatar"
    }
    ```
-   **Response (200 OK):**
    ```json
    {
      "status": "success",
      "message": "Profile updated successfully."
    }
    ```

#### `GET /users`

-   **Mô tả:** Lấy danh sách tất cả người dùng, có phân trang.
-   **Xác thực:** Admin.
-   **Query Params:**
    -   `page` (int, optional, default: 1): Số trang.
    -   `limit` (int, optional, default: 20): Số mục trên mỗi trang.
-   **Response (200 OK):**
    ```json
    {
      "data": [
        {
          "id": 1,
          "display_name": "John Doe",
          "email": "john.doe@example.com",
          "role": "user",
          "created_at": "2023-10-27T10:00:00Z"
        }
      ],
      "pagination": {
        "current_page": 1,
        "total_pages": 10,
        "total_items": 198,
        "limit": 20
      }
    }
    ```

#### `PATCH /users/{id}`

-   **Mô tả:** Cập nhật vai trò của một người dùng.
-   **Xác thực:** Admin.
-   **Body:**
    ```json
    {
      "role": "admin"
    }
    ```
-   **Response (200 OK):**
    ```json
    {
      "status": "success",
      "message": "User role updated successfully."
    }
    ```

---

### FR2 & FR3: Quản lý và Trải nghiệm Quiz

#### `GET /quizzes`

-   **Mô tả:** Lấy danh sách các bài quiz (dạng tóm tắt), hỗ trợ lọc, tìm kiếm và phân trang.
-   **Query Params:**
    -   `category_id` (int, optional): Lọc theo ID chủ đề.
    -   `difficulty` (string, optional): Lọc theo độ khó (`easy`, `medium`, `hard`).
    -   `q` (string, optional): Tìm kiếm theo tiêu đề.
    -   `page` (int, optional, default: 1): Số trang.
    -   `limit` (int, optional, default: 10): Số mục trên mỗi trang.
    -   `sort_by` (string, optional): Trường để sắp xếp. Các giá trị hợp lệ: `created_at` (mặc định), `like_count`, `comment_count`, `question_count`, `title`.
    -   `order` (string, optional): Hướng sắp xếp. Các giá trị hợp lệ: `asc` (tăng dần), `desc` (giảm dần, mặc định).
-   **Response (200 OK):** `PaginatedQuizzesDto`
    ```json
    {
      "pagination": {
        "current_page": 1,
        "total_pages": 5,
        "total_items": 50,
        "limit": 10
      },
      "data": [
        {
          "id": 1,
          "title": "Basics of Arrays",
          "difficulty": "easy",
          "question_count": 15,
          "like_count": 128,
          "comment_count": 23,
          "category_id": 1,
          "category_name": "Data Structures"
        }
      ]
    }
    ```

#### `POST /quizzes`

-   **Mô tả:** (Admin) Tạo một bài quiz mới.
-   **Body:** `CreateQuizDto`
-   **Response (201 Created):** `QuizDetailDto` (trả về chi tiết quiz vừa tạo).

#### `GET /quizzes/{id}`

-   **Mô tả:** Lấy thông tin chi tiết một bài quiz.
-   **Response (200 OK):** `QuizDetailDto`
    ```json
    {
      "id": 1,
      "title": "Basics of Arrays",
      "description": "A quiz to test fundamental knowledge of arrays.",
      "difficulty": "easy",
      "question_count": 15,
      "like_count": 128,
      "comment_count": 23,
      "category_id": 1,
      "category_name": "Data Structures",
      "creator_id": 1,
      "creator_display_name": "John Doe",
      "creator_avatar_url": "https://example.com/avatar.png",
      "created_at": "2023-10-27T10:00:00Z",
      "updated_at": "2023-10-27T10:00:00Z"
    }
    ```

#### `PUT /quizzes/{id}`

-   **Mô tả:** (Admin) Cập nhật một bài quiz.
-   **Body:** `UpdateQuizDto`.
-   **Response (200 OK):** `QuizDetailDto`.

#### `DELETE /quizzes/{id}`

-   **Mô tả:** (Admin) Xóa một bài quiz.
-   **Response (204 No Content):**

---

#### `GET /categories`

-   **Mô tả:** Lấy danh sách tất cả các chủ đề, có phân trang.
-   **Query Params:**
    -   `name_pattern` (string, optional): Tìm kiếm theo tên chủ đề.
    -   `page` (int, optional, default: 1): Số trang.
    -   `limit` (int, optional, default: 10): Số mục trên mỗi trang.
-   **Response (200 OK):** `PaginatedCategoriesDto`
    ```json
    {
      "pagination": {
        "current_page": 1,
        "total_pages": 3,
        "total_items": 25,
        "limit": 10
      },
      "data": [
        {
          "id": 1,
          "name": "Data Structures",
          "image_url": "url-to-image",
          "description": "A collection of quizzes about fundamental data structures."
        }
      ]
    }
    ```

#### `POST /categories`

-   **Mô tả:** (Admin) Tạo một chủ đề mới.
-   **Body:** `CreateCategoryDto`.
-   **Response (201 Created):** `CategoryDto`.

#### `PUT /categories/{id}`

-   **Mô tả:** (Admin) Cập nhật một chủ đề.
-   **Body:** `UpdateCategoryDto`.
-   **Response (200 OK):** `CategoryDto`.

#### `DELETE /categories/{id}`

-   **Mô tả:** (Admin) Xóa một chủ đề.
-   **Response (204 No Content):**

---

#### `GET /quizzes/{id}/questions`

-   **Mô tả:** Lấy danh sách câu hỏi cho một bài quiz (dành cho người dùng làm bài).
-   **Response (200 OK):** `Vec<PublicQuestionDto>`
    ```json
    [
      {
        "id": 101,
        "type": "single_choice",
        "content": "What is a Stack?",
        "image_url": null,
        "options": [
          { "content": "LIFO", "image_url": null },
          { "content": "FIFO", "image_url": null }
        ]
      }
    ]
    ```
    *Lưu ý: Response này **không** chứa đáp án.*

---

### FR4: Tương tác & Cộng đồng

#### `GET /comments`

-   **Mô tả:** Lấy danh sách bình luận của một quiz, có phân trang.
-   **Query Params:** `quiz_id` (bắt buộc), `page`, `limit`.
-   **Response (200 OK):** `PaginatedCommentsDto`
    ```json
    {
      "pagination": { ... },
      "data": [
        {
          "id": 1,
          "content": "Great quiz!",
          "created_at": "2023-10-28T11:00:00Z",
          "user_id": 2,
          "user_display_name": "Jane Doe",
          "user_avatar_url": "...",
          "like_count": 15,
          "is_liked_by_user": true
        }
      ]
    }
    ```

#### `POST /comments`

-   **Mô tả:** Gửi một bình luận mới.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:** `CreateCommentDto`
-   **Response (201- Created):** `CommentDto`.

#### `DELETE /comments/{id}`

-   **Mô tả:** Xóa một bình luận (chỉ chủ sở hữu hoặc admin).
-   **Xác thực:** Yêu cầu JWT.
-   **Response (204 No Content):**

---

#### `POST /likes/quiz`

-   **Mô tả:** Thích một bài quiz.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:** `CreateLikeDto` (`{ "target_id": <quiz_id> }`)
-   **Response (201 Created):**

#### `DELETE /likes/quiz`

-   **Mô tả:** Bỏ thích một bài quiz.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:** `DeleteLikeDto` (`{ "target_id": <quiz_id> }`).
-   **Response (204 No Content):**

#### `POST /likes/comment`

-   **Mô tả:** Thích một bình luận.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:** `CreateLikeDto` (`{ "target_id": <comment_id> }`)
-   **Response (201 Created):**

#### `DELETE /likes/comment`

-   **Mô tả:** Bỏ thích một bình luận.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:** `DeleteLikeDto` (`{ "target_id": <comment_id> }`).
-   **Response (204 No Content):**

---

### FR5: Hồ sơ cá nhân & Lịch sử

#### `POST /submissions`

-   **Mô tả:** Nộp bài quiz đã làm để chấm điểm.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:** `SubmissionRequestDto`
    ```json
    {
      "quiz_id": 1,
      "answers": [
        { "question_id": 101, "data": { "option_index": 0 } },
        { "question_id": 102, "data": { "entry": "O(n)" } }
      ]
    }
    ```
-   **Response (201 Created):** `SubmissionDetailDto`

#### `GET /submissions/{id}`

-   **Mô tả:** Lấy chi tiết kết quả của một lần nộp bài.
-   **Xác thực:** Yêu cầu JWT (người dùng chỉ xem được bài của mình, admin xem được mọi bài).
-   **Response (200 OK):** `SubmissionDetailDto`

#### `GET /users/me/submissions`

-   **Mô tả:** Lấy lịch sử làm bài của người dùng hiện tại, có phân trang.
-   **Xác thực:** Yêu cầu JWT.
-   **Query Params:** `page`, `limit`.
-   **Response (200 OK):** `PaginatedSubmissionsDto`
