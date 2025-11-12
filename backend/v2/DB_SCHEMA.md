# Đặc tả Schema Database (PostgreSQL)

Tài liệu này mô tả cấu trúc database cho dự án csquizz, dựa trên file migration `up.sql`.

---

## 1. Các kiểu ENUM

Các kiểu dữ liệu ENUM được sử dụng để định nghĩa các giá trị cố định cho một số trường.

### `user_role`

-   **Mô tả:** Vai trò của người dùng trong hệ thống.
-   **Giá trị:**
    -   `'user'`
    -   `'admin'`

### `quiz_difficulty`

-   **Mô tả:** Độ khó của một bài quiz.
-   **Giá trị:**
    -   `'easy'`
    -   `'medium'`
    -   `'hard'`

### `question_type`

-   **Mô tả:** Loại câu hỏi.
-   **Giá trị:**
    -   `'single_choice'` (Trắc nghiệm một lựa chọn)
    -   `'multiple_choice'` (Trắc nghiệm nhiều lựa chọn)
    -   `'text_entry'` (Điền từ/Trả lời ngắn)

---

## 2. Các Bảng (Tables)

### Bảng `users`

-   **Mô tả:** Lưu trữ thông tin người dùng.

| Tên cột | Kiểu dữ liệu | Ràng buộc | Mô tả |
| :------ | :----------- | :-------- | :---- |
| `usr_id` | `SERIAL` | `PRIMARY KEY` | ID duy nhất của người dùng. |
| `usr_google_id` | `TEXT` | `UNIQUE` | ID từ Google OAuth nếu đăng nhập bằng Google. |
| `usr_display_name` | `VARCHAR(50)` | `NOT NULL` | Tên hiển thị của người dùng. |
| `usr_email` | `VARCHAR(100)` | `UNIQUE`, `NOT NULL` | Địa chỉ email duy nhất của người dùng. |
| `usr_password_hash` | `TEXT` | | Hash mật khẩu của người dùng. |
| `usr_avatar_url` | `TEXT` | | URL ảnh đại diện của người dùng. |
| `usr_role` | `user_role` | `NOT NULL`, `DEFAULT 'user'` | Vai trò của người dùng. |
| `usr_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Thời điểm tạo tài khoản. |

### Bảng `categories`

-   **Mô tả:** Lưu trữ thông tin các chủ đề (category) của quiz.

| Tên cột | Kiểu dữ liệu | Ràng buộc | Mô tả |
| :------ | :----------- | :-------- | :---- |
| `cat_id` | `SERIAL` | `PRIMARY KEY` | ID duy nhất của chủ đề. |
| `cat_name` | `VARCHAR(50)` | `UNIQUE`, `NOT NULL` | Tên chủ đề. |
| `cat_image_url` | `TEXT` | | URL ảnh đại diện cho chủ đề. |
| `cat_description` | `TEXT` | | Mô tả ngắn về chủ đề. |

### Bảng `quizzes`

-   **Mô tả:** Lưu trữ thông tin các bài quiz.

| Tên cột | Kiểu dữ liệu | Ràng buộc | Mô tả |
| :------ | :----------- | :-------- | :---- |
| `qz_id` | `SERIAL` | `PRIMARY KEY` | ID duy nhất của quiz. |
| `qz_title` | `VARCHAR(200)` | `NOT NULL` | Tiêu đề của quiz. |
| `qz_description` | `TEXT` | | Mô tả chi tiết về quiz. |
| `qz_difficulty` | `quiz_difficulty` | `NOT NULL` | Độ khó của quiz. |
| `qz_category_id` | `INT` | `REFERENCES categories(cat_id) ON DELETE SET NULL` | ID của chủ đề mà quiz thuộc về. |
| `qz_creator_id` | `INT` | `REFERENCES users(usr_id) ON DELETE SET NULL` | ID của người tạo quiz. |
| `qz_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Thời điểm tạo quiz. |
| `qz_updated_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Thời điểm cập nhật quiz gần nhất. |

### Bảng `questions`

-   **Mô tả:** Lưu trữ thông tin các câu hỏi trong quiz.

| Tên cột | Kiểu dữ liệu | Ràng buộc | Mô tả |
| :------ | :----------- | :-------- | :---- |
| `qs_id` | `SERIAL` | `PRIMARY KEY` | ID duy nhất của câu hỏi. |
| `qs_type` | `question_type` | `NOT NULL` | Loại câu hỏi (single_choice, multiple_choice, text_entry). |
| `qs_content` | `TEXT` | `NOT NULL` | Nội dung câu hỏi. |
| `qs_image_url` | `TEXT` | | URL ảnh minh họa cho câu hỏi. |
| `qs_key` | `JSONB` | `NOT NULL` | Chứa thông tin đáp án đúng và giải thích (cấu trúc chi tiết bên dưới). |
| `qs_quiz_id` | `INT` | `REFERENCES quizzes(qz_id) ON DELETE CASCADE` | ID của quiz mà câu hỏi thuộc về. |
| `qs_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Thời điểm tạo câu hỏi. |

#### Cấu trúc `qs_key` (JSONB)

-   **`single_choice` / `multiple_choice`:**
    ```json
    {
      "keys": [
        {
          "content": "string",
          "image_url": "string?",
          "is_correct": "boolean",
          "explanation": "string?"
        },
        // ...
      ]
    }
    ```
-   **`text_entry`:**
    ```json
    {
      "correct_entry": "string",
      "explanation": "string?"
    }
    ```

### Bảng `submission_results`

-   **Mô tả:** Lưu trữ kết quả tổng quan của một lần người dùng nộp bài quiz.

| Tên cột | Kiểu dữ liệu | Ràng buộc | Mô tả |
| :------ | :----------- | :-------- | :---- |
| `sub_id` | `SERIAL` | `PRIMARY KEY` | ID duy nhất của kết quả nộp bài. |
| `sub_user_id` | `INT` | `REFERENCES users(usr_id) ON DELETE CASCADE` | ID của người dùng đã nộp bài. |
| `sub_quiz_id` | `INT` | `REFERENCES quizzes(qz_id) ON DELETE CASCADE` | ID của quiz đã được nộp. |
| `sub_score` | `FLOAT` | `NOT NULL`, `CHECK (sub_score >= 0)` | Điểm số đạt được (0-100). |
| `sub_submitted_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Thời điểm nộp bài. |

### Bảng `answers`

-   **Mô tả:** Lưu trữ chi tiết câu trả lời của người dùng cho từng câu hỏi trong một lần nộp bài.

| Tên cột | Kiểu dữ liệu | Ràng buộc | Mô tả |
| :------ | :----------- | :-------- | :---- |
| `ans_id` | `SERIAL` | `PRIMARY KEY` | ID duy nhất của câu trả lời. |
| `ans_result_id` | `INT` | `REFERENCES submission_results(sub_id) ON DELETE CASCADE` | ID của kết quả nộp bài mà câu trả lời này thuộc về. |
| `ans_question_id` | `INT` | `REFERENCES questions(qs_id) ON DELETE CASCADE` | ID của câu hỏi được trả lời. |
| `ans_data` | `JSONB` | `NOT NULL` | Dữ liệu câu trả lời của người dùng (cấu trúc chi tiết bên dưới). |

#### Cấu trúc `ans_data` (JSONB)

-   **`single_choice`:**
    ```json
    {
      "option_index": "number" // Index của lựa chọn được chọn
    }
    ```
-   **`multiple_choice`:**
    ```json
    {
      "choices": [
        { "option_index": "number" },
        // ...
      ]
    }
    ```
-   **`text_entry`:**
    ```json
    {
      "entry": "string" // Nội dung người dùng điền vào
    }
    ```

### Bảng `quiz_likes`

-   **Mô tả:** Ghi nhận lượt thích của người dùng đối với một bài quiz.

| Tên cột | Kiểu dữ liệu | Ràng buộc | Mô tả |
| :------ | :----------- | :-------- | :---- |
| `qzlk_user_id` | `INT` | `REFERENCES users(usr_id) ON DELETE CASCADE` | ID của người dùng đã thích. |
| `qzlk_quiz_id` | `INT` | `REFERENCES quizzes(qz_id) ON DELETE CASCADE` | ID của quiz được thích. |
| `CONSTRAINT unique_quiz_like` | | `UNIQUE(qzlk_user_id, qzlk_quiz_id)` | Đảm bảo mỗi người dùng chỉ thích một quiz một lần. |

### Bảng `comments`

-   **Mô tả:** Lưu trữ các bình luận của người dùng về một bài quiz.

| Tên cột | Kiểu dữ liệu | Ràng buộc | Mô tả |
| :------ | :----------- | :-------- | :---- |
| `cmt_id` | `SERIAL` | `PRIMARY KEY` | ID duy nhất của bình luận. |
| `cmt_user_id` | `INT` | `REFERENCES users(usr_id) ON DELETE CASCADE` | ID của người dùng đã bình luận. |
| `cmt_quiz_id` | `INT` | `REFERENCES quizzes(qz_id) ON DELETE CASCADE` | ID của quiz được bình luận. |
| `cmt_content` | `TEXT` | `NOT NULL` | Nội dung bình luận. |
| `cmt_created_at` | `TIMESTAMPTZ` | `DEFAULT NOW()` | Thời điểm tạo bình luận. |

### Bảng `comment_likes`

-   **Mô tả:** Ghi nhận lượt thích của người dùng đối với một bình luận.

| Tên cột | Kiểu dữ liệu | Ràng buộc | Mô tả |
| :------ | :----------- | :-------- | :---- |
| `cmtlk_user_id` | `INT` | `REFERENCES users(usr_id) ON DELETE CASCADE` | ID của người dùng đã thích bình luận. |
| `cmtlk_comment_id` | `INT` | `REFERENCES comments(cmt_id) ON DELETE CASCADE` | ID của bình luận được thích. |
| `CONSTRAINT unique_comment_like` | | `UNIQUE(cmtlk_user_id, cmtlk_comment_id)` | Đảm bảo mỗi người dùng chỉ thích một bình luận một lần. |
