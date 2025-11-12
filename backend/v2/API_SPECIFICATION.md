# Đặc tả API (API Specification) cho Dự án csquizz

-   **Phiên bản:** 1.0
-   **Định dạng:** JSON

## 1. Nguyên tắc thiết kế

-   **RESTful:** API được thiết kế theo các nguyên tắc REST.
-   **Stateless:** Mọi request từ client phải chứa đủ thông tin để server có thể hiểu và xử lý (thường là qua JWT).
-   **Không lồng ghép tài nguyên (No-nesting):** Các endpoint được giữ phẳng. Để truy vấn tài nguyên con, sử dụng query parameter (ví dụ: `GET /questions?quiz_id=...`).
-   **Authentication:** Sử dụng `Bearer Token` (JWT) trong header `Authorization` cho các request cần xác thực.

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

### FR2: Quản lý Nội dung (Admin)

#### `GET /categories`

-   **Mô tả:** Lấy danh sách tất cả các chủ đề.
-   **Response (200 OK):**
    ```json
    [
      {
        "id": 1,
        "name": "Data Structures",
        "image_url": "url-to-image",
        "description": "..."
      }
    ]
    ```

#### `POST /categories`

-   **Mô tả:** Tạo một chủ đề mới.
-   **Xác thực:** Admin.
-   **Body:**
    ```json
    {
      "name": "Algorithms",
      "image_url": "url-to-image",
      "description": "..."
    }
    ```
-   **Response (201- Created):** (Trả về đối tượng vừa tạo)

#### `PUT /categories/{id}`

-   **Mô tả:** Cập nhật một chủ đề.
-   **Xác thực:** Admin.
-   **Response (200 OK):** (Trả về đối tượng vừa cập nhật)

#### `DELETE /categories/{id}`

-   **Mô tả:** Xóa một chủ đề.
-   **Xác thực:** Admin.
-   **Response (204 No Content):**

---

#### `GET /quizzes`

-   **Mô tả:** Lấy danh sách các bài quiz, hỗ trợ lọc, tìm kiếm và phân trang.
-   **Query Params:**
    -   `category_id` (int): Lọc theo ID chủ đề.
    -   `difficulty` (string): Lọc theo độ khó (`easy`, `medium`, `hard`).
    -   `q` (string): Tìm kiếm theo tiêu đề.
    -   `page` (int, optional, default: 1): Số trang.
    -   `limit` (int, optional, default: 10): Số mục trên mỗi trang.
-   **Response (200 OK):**
    ```json
    {
      "data": [
        {
          "id": 1,
          "title": "Basics of Arrays",
          "description": "...",
          "difficulty": "easy",
          "category_id": 1,
          "creator_id": 1
        }
      ],
      "pagination": {
        "current_page": 1,
        "total_pages": 5,
        "total_items": 50,
        "limit": 10
      }
    }
    ```

#### `POST /quizzes`

-   **Mô tả:** Tạo một bài quiz mới.
-   **Xác thực:** Admin.
-   **Body:**
    ```json
    {
      "title": "Advanced Sorting Algorithms",
      "description": "...",
      "difficulty": "hard",
      "category_id": 2
    }
    ```
-   **Response (201 Created):** (Trả về đối tượng quiz vừa tạo)

#### `GET /quizzes/{id}`

-   **Mô tả:** Lấy thông tin chi tiết một bài quiz.
-   **Response (200 OK):** (Tương tự một object trong `GET /quizzes`)

#### `PUT /quizzes/{id}`

-   **Mô tả:** Cập nhật một bài quiz.
-   **Xác thực:** Admin.
-   **Response (200 OK):** (Trả về đối tượng quiz vừa cập nhật)

#### `DELETE /quizzes/{id}`

-   **Mô tả:** Xóa một bài quiz.
-   **Xác thực:** Admin.
-   **Response (204 No Content):**

---

#### `GET /questions`

-   **Mô tả:** Lấy danh sách câu hỏi cho một bài quiz, có phân trang.
-   **Query Params:**
    -   `quiz_id` (int, **bắt buộc**): ID của bài quiz.
    -   `page` (int, optional, default: 1): Số trang.
    -   `limit` (int, optional, default: 10): Số mục trên mỗi trang.
-   **Response (200 OK):**
    ```json
    {
      "data": [
        {
          "id": 101,
          "type": "single_choice",
          "content": "What is a Stack?",
          "image_url": null,
          "quiz_id": 1,
          "key": { // Đáp án và giải thích
            "options": [
              {"content": "LIFO", "is_correct": true, "explanation": "Stack is Last-In, First-Out."},
              {"content": "FIFO", "is_correct": false}
            ]
          }
        }
      ],
      "pagination": {
        "current_page": 1,
        "total_pages": 2,
        "total_items": 15,
        "limit": 10
      }
    }
    ```

#### `POST /questions`

-   **Mô tả:** Tạo một câu hỏi mới và gán vào một quiz.
-   **Xác thực:** Admin.
-   **Body:**
    ```json
    {
      "quiz_id": 1,
      "type": "single_choice",
      "content": "What is a Queue?",
      "key": {
        "options": [
          {"content": "LIFO", "is_correct": false},
          {"content": "FIFO", "is_correct": true, "explanation": "Queue is First-In, First-Out."}
        ]
      }
    }
    ```
-   **Response (201 Created):** (Trả về đối tượng câu hỏi vừa tạo)

### FR3: Trải nghiệm Làm Quiz

#### `POST /submissions`

-   **Mô tả:** Nộp bài quiz đã làm để chấm điểm.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:**
    ```json
    {
      "quiz_id": 1,
      "answers": [
        {
          "question_id": 101,
          "data": { "option_index": 0 } // UserChoice
        },
        {
          "question_id": 102,
          "data": { "entry": "O(n)" } // UserEntry
        }
      ]
    }
    ```
-   **Response (201 Created):**
    ```json
    {
      "id": 1,
      "user_id": 1,
      "quiz_id": 1,
      "score": 50.0,
      "submitted_at": "2023-10-27T11:00:00Z",
      "details": [ // Chi tiết từng câu trả lời
        {
          "question_id": 101,
          "user_answer": { "option_index": 0 },
          "correct_answer": { "option_index": 0 },
          "is_correct": true,
          "explanation": "Stack is Last-In, First-Out."
        }
      ]
    }
    ```

#### `GET /submissions/{id}`

-   **Mô tả:** Lấy chi tiết kết quả của một lần nộp bài.
-   **Xác thực:** Yêu cầu JWT (người dùng chỉ xem được bài của mình, admin xem được mọi bài).
-   **Response (200 OK):** (Tương tự response của `POST /submissions`)

### FR4: Tương tác & Cộng đồng

#### `POST /quiz-likes`

-   **Mô tả:** Thích một bài quiz.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:**
    ```json
    {
      "quiz_id": 1
    }
    ```
-   **Response (201 Created):**
    ```json
    { "status": "success", "message": "Quiz liked." }
    ```

#### `DELETE /quiz-likes`

-   **Mô tả:** Bỏ thích một bài quiz.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:**
    ```json
    {
      "quiz_id": 1
    }
    ```
-   **Response (204 No Content):**

#### `GET /comments`

-   **Mô tả:** Lấy danh sách bình luận của một quiz, có phân trang.
-   **Query Params:**
    -   `quiz_id` (int, **bắt buộc**): ID của bài quiz.
    -   `page` (int, optional, default: 1): Số trang.
    -   `limit` (int, optional, default: 10): Số mục trên mỗi trang.
-   **Response (200 OK):**
    ```json
    {
      "data": [
        {
          "id": 1,
          "user_id": 2,
          "user_display_name": "Jane Doe",
          "content": "Great quiz!",
          "created_at": "..."
        }
      ],
      "pagination": {
        "current_page": 1,
        "total_pages": 3,
        "total_items": 25,
        "limit": 10
      }
    }
    ```

#### `POST /comments`

-   **Mô tả:** Gửi một bình luận mới.
-   **Xác thực:** Yêu cầu JWT.
-   **Body:**
    ```json
    {
      "quiz_id": 1,
      "content": "This was very helpful, thanks!"
    }
    ```
-   **Response (201 Created):** (Trả về đối tượng comment vừa tạo)

### FR5: Hồ sơ cá nhân & Lịch sử

#### `GET /users/me/submissions`

-   **Mô tả:** Lấy lịch sử làm bài của người dùng hiện tại, có phân trang.
-   **Xác thực:** Yêu cầu JWT.
-   **Query Params:**
    -   `page` (int, optional, default: 1): Số trang.
    -   `limit` (int, optional, default: 10): Số mục trên mỗi trang.
-   **Response (200 OK):**
    ```json
    {
      "data": [
        {
          "submission_id": 1,
          "quiz_id": 1,
          "quiz_title": "Basics of Arrays",
          "score": 85.0,
          "submitted_at": "2023-10-26T14:30:00Z"
        }
      ],
      "pagination": {
        "current_page": 1,
        "total_pages": 4,
        "total_items": 38,
        "limit": 10
      }
    }
    ```