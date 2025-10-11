# API Specifications

**Base URL:** `/api`

---

## 1. Authentication (`/auth`)

### 1.1. Đăng ký người dùng mới

-   **Endpoint:** `POST /api/auth/register`
-   **Mô tả:** Tạo một tài khoản người dùng mới.
-   **Request Body:** `application/json`
    ```json
    {
        "username": "string", // Bắt buộc, duy nhất
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
    -   `400 Bad Request`: Dữ liệu đầu vào không hợp lệ (thiếu trường, email sai định dạng...).
    -   `409 Conflict`: `username` hoặc `email` đã tồn tại.

### 1.2. Đăng nhập

-   **Endpoint:** `POST /api/auth/login`
-   **Mô tả:** Xác thực người dùng và trả về một JSON Web Token (JWT).
-   **Request Body:** `application/json`
    ```json
    {
        "email": "string",    // Bắt buộc
        "password": "string"  // Bắt buộc
    }
    ```
-   **Success Response:** `200 OK`
    ```json
    {
        "token": "string", // Chuỗi JWT
        "user": {
            "id": "number",
            "username": "string",
            "role": "string"
        }
    }
    ```
-   **Error Responses:**
    -   `401 Unauthorized`: Sai `email` hoặc `password`.

### 1.3. Lấy thông tin người dùng hiện tại

-   **Endpoint:** `GET /api/auth/me`
-   **Mô tả:** Lấy thông tin của người dùng đang đăng nhập dựa trên token.
-   **Authentication:** Yêu cầu `Bearer Token` trong header `Authorization`.
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
    -   `401 Unauthorized`: Token không hợp lệ hoặc hết hạn.

---

## 2. Quizzes (`/quizzes`)

### 2.1. Lấy danh sách các quiz

-   **Endpoint:** `GET /api/quizzes`
-   **Mô tả:** Lấy danh sách các bài quiz có sẵn, hỗ trợ tìm kiếm và lọc.
-   **Query Parameters:**
    -   `title_pattern`: `string` (optional) - Tìm kiếm theo tiêu đề quiz.
    -   `category`: `string` (optional) - Lọc theo chủ đề.
    -   `difficulty`: `string` (optional) - Lọc theo độ khó (`easy`, `medium`, `hard`).
    -   `completed_by`: `number` (optional)
    -   `page`: `number`
    -   `size`: `number`
-   **Success Response:** `200 OK`
    ```json
    {
        "quizzes": [
            {
                "id": "number",
                "title": "string",
                "description": "string" (optional),
                "category": "string",
                "difficulty": "string" (optional),
                "create_by": "string" (optional),
                "created_at": "date"
            }
        ]
    }
    ```

### 2.2. Lấy thông tin 1 quiz

-   **Endpoint:** `GET /api/quizzes/{id}`
-   **Mô tả:** Lấy thông tin 1 quiz.
-   **Success Response:** `200 OK`
    ```json
    {
        "id": "number",
        "title": "string",
        "description": "string" (optional),
        "category": "string",
        "difficulty": "string" (optional),
        "create_by": "string" (optional),
        "created_at": "date"
    }
    ```

### 2.3. Lấy chi tiết một bài quiz (để làm bài)

-   **Endpoint:** `GET /api/quizzes/{id}/questions`
-   **Mô tả:** Lấy thông tin chi tiết của một quiz, bao gồm danh sách câu hỏi và các lựa chọn (không kèm đáp án đúng).
-   **Path Parameters:**
    -   `page`: `number`
    -   `size`: `number`
-   **Success Response:** `200 OK`
    ```json
    {
        "questions": [
            {
                "id": "number",
                "form": "string",
                "text": "string",
                "image_url": "string" (optional),
                "options": [
                    { "id": "number", "text": "string" },
                    { "id": "number", "text": "string" }
                ]
            }
        ]
    }
    ```
-   **Error Responses:**
    -   `404 Not Found`: Không tìm thấy quiz với id tương ứng.

### 2.4. Nộp bài và chấm điểm

-   **Endpoint:** `POST /api/quizzes/{id}/submit`
-   **Mô tả:** Nhận bài làm của người dùng, chấm điểm và trả về kết quả chi tiết.
-   **Authentication:** Có thể không bắt buộc (cho guest) hoặc bắt buộc (để lưu kết quả cho user).
-   **Request Body:** `application/json`
    ```json
    {
        "user_id": "number",
        "answers": [
            {
                "question_id": "number",
                "selected_option_ids": ["number"]
            }
        ]
    }
    ```
-   **Success Response:** `200 OK`
    ```json
    {
        "result_id": "number",
        "score": "number",
        "total_questions": "number",
        "correct_answers": "number",
        "details": [
            {
                "question_id": "number",
                "selected_option_id": "number",
                "correct_option_id": "number",
                "is_correct": "boolean",
                "explanation": "string" (optional)
            }
        ]
    }
    ```

---

## 3. Categories (`/categories`)

### 3.1. Lấy danh sách category

-   **Endpoint:** `GET /api/categories`
-   **Mô tả:** Lấy thông tin chi tiết về tất cả các category của quiz.
-   **Path Parameters:**
    -   `page`: `number`
    -   `size`: `number`
-   **Success Response:** `200 OK`
    ```json
    {
        "id": "number",
        "name": "string",
        "image_url": "string" (optional),
        "description": "string" (optional)
    }
    ```
-   **Error Responses:**
    -   `404 Not Found`: Không tìm thấy quiz với id tương ứng.

---

## 4. User (`/user`)

### 4.1. Lấy lịch sử làm bài

-   **Endpoint:** `GET /api/user/results`
-   **Mô tả:** Lấy danh sách các kết quả quiz của người dùng đang đăng nhập.
-   **Authentication:** Yêu cầu `Bearer Token`.
-   **Success Response:** `200 OK`
    ```json
    {
        "results": [
            {
                "result_id": "number",
                "quiz_id": "number",
                "quiz_title": "string",
                "score": "number",
                "total_questions": "number",
                "submitted_at": "string"
            }
        ]
    }
    ```
-   **Error Responses:**
    -   `401 Unauthorized`: Token không hợp lệ.

---

## 5. Admin (`/admin`)

-   `POST /api/admin/quizzes`: Tạo một quiz mới.
-   `PUT /api/admin/quizzes/{id}`: Cập nhật thông tin một quiz.
-   `DELETE /api/admin/quizzes/{id}`: Xóa một quiz.
-   `POST /api/admin/quizzes/{quizId}/questions`: Thêm một câu hỏi mới vào quiz.
-   `PUT /api/admin/questions/{questionId}`: Cập nhật một câu hỏi.
-   `DELETE /api/admin/questions/{questionId}`: Xóa một câu hỏi.
