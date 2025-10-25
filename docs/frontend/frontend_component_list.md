# Danh sách các Component Frontend - Quiz-Bank

Tài liệu này liệt kê các thành phần giao diện người dùng (UI components) chính sẽ được sử dụng và tái sử dụng trong ứng dụng Quiz-Bank. Mục tiêu là đảm bảo tính nhất quán trong thiết kế và tăng tốc độ phát triển.

Các component sẽ được xây dựng dựa trên React, Tailwind CSS và Shadcn UI.

## 1. Component Layout & Cấu trúc

### 1.1. Navbar
-   **Mô tả:** Thanh điều hướng chính ở đầu trang, chứa logo, các liên kết điều hướng và thông tin người dùng/nút đăng nhập/đăng ký.
-   **Sử dụng:** Trên tất cả các trang.
-   **Thuộc tính chính:** `isAuthenticated` (hiển thị trạng thái đăng nhập), `user` (thông tin người dùng).
-   **Nguồn:** Custom / Kết hợp Shadcn UI (Dropdown Menu).

### 1.2. Footer
-   **Mô tả:** Chân trang, chứa thông tin bản quyền và các liên kết phụ.
-   **Sử dụng:** Trên tất cả các trang.
-   **Nguồn:** Custom.

### 1.3. Container
-   **Mô tả:** Component bao bọc nội dung chính của trang, giới hạn chiều rộng để nội dung dễ đọc và căn giữa.
-   **Sử dụng:** Bao bọc nội dung chính của hầu hết các trang.
-   **Nguồn:** Custom (sử dụng Tailwind CSS).

## 2. Component Form

### 2.1. Button
-   **Mô tả:** Nút bấm để thực hiện các hành động.
-   **Sử dụng:** Nộp form, điều hướng, kích hoạt chức năng.
-   **Thuộc tính chính:** `variant` (primary, secondary, outline, ghost, link), `size` (default, sm, lg, icon), `disabled`, `onClick`.
-   **Nguồn:** Shadcn UI.

### 2.2. Input
-   **Mô tả:** Trường nhập liệu văn bản một dòng.
-   **Sử dụng:** Form đăng nhập, đăng ký, tìm kiếm, nhập câu trả lời.
-   **Thuộc tính chính:** `type` (text, email, password, number), `placeholder`, `value`, `onChange`, `disabled`.
-   **Nguồn:** Shadcn UI.

### 2.3. Textarea
-   **Mô tả:** Trường nhập liệu văn bản nhiều dòng.
-   **Sử dụng:** Nhập câu trả lời dài, mô tả.
-   **Thuộc tính chính:** `placeholder`, `value`, `onChange`, `rows`.
-   **Nguồn:** Shadcn UI.

### 2.4. Checkbox
-   **Mô tả:** Hộp kiểm cho phép chọn nhiều tùy chọn.
-   **Sử dụng:** Câu hỏi trắc nghiệm nhiều lựa chọn.
-   **Thuộc tính chính:** `checked`, `onCheckedChange`, `disabled`.
-   **Nguồn:** Shadcn UI.

### 2.5. RadioGroup (Radio Button)
-   **Mô tả:** Nhóm các nút radio, chỉ cho phép chọn một tùy chọn duy nhất.
-   **Sử dụng:** Câu hỏi trắc nghiệm một lựa chọn.
-   **Thuộc tính chính:** `value`, `onValueChange`, `disabled`.
-   **Nguồn:** Shadcn UI.

### 2.6. Label
-   **Mô tả:** Nhãn cho các trường form.
-   **Sử dụng:** Gắn liền với Input, Checkbox, RadioGroup.
-   **Nguồn:** Shadcn UI.

## 3. Component Hiển thị dữ liệu

### 3.1. Card
-   **Mô tả:** Component dạng thẻ để hiển thị thông tin một cách có tổ chức.
-   **Sử dụng:** Hiển thị chủ đề quiz, kết quả tóm tắt.
-   **Nguồn:** Shadcn UI.

### 3.2. QuizItem Card
-   **Mô tả:** Một dạng Card cụ thể để hiển thị thông tin về một bài quiz (tên, mô tả, nút bắt đầu).
-   **Sử dụng:** Trang chủ (danh sách quiz).
-   **Nguồn:** Custom (dựa trên Shadcn UI Card).

### 3.3. Table
-   **Mô tả:** Bảng để hiển thị dữ liệu có cấu trúc (hàng và cột).
-   **Sử dụng:** Lịch sử làm bài, quản lý ngân hàng câu hỏi (trang Admin).
-   **Nguồn:** Shadcn UI.

### 3.4. Badge
-   **Mô tả:** Nhãn nhỏ hiển thị trạng thái hoặc phân loại.
-   **Sử dụng:** Hiển thị độ khó của quiz (Dễ, Trung bình, Khó), trạng thái (Đúng, Sai).
-   **Nguồn:** Shadcn UI.

### 3.5. Progress Bar
-   **Mô tả:** Thanh hiển thị tiến độ hoàn thành.
-   **Sử dụng:** Trang làm Quiz (tiến độ làm bài).
-   **Nguồn:** Shadcn UI.

### 3.6. Alert
-   **Mô tả:** Hộp thông báo hiển thị các tin nhắn quan trọng (thành công, lỗi, cảnh báo).
-   **Sử dụng:** Thông báo lỗi đăng nhập, thông báo nộp bài thành công.
-   **Nguồn:** Shadcn UI.

## 4. Component Điều hướng & Phản hồi

### 4.1. Pagination
-   **Mô tả:** Component phân trang để điều hướng qua các danh sách dài.
-   **Sử dụng:** Trang chủ (nếu có nhiều chủ đề quiz).
-   **Nguồn:** Shadcn UI.

### 4.2. Dropdown Menu
-   **Mô tả:** Menu thả xuống khi click vào một nút hoặc biểu tượng.
-   **Sử dụng:** Menu người dùng trong Navbar.
-   **Nguồn:** Shadcn UI.

### 4.3. Toast
-   **Mô tả:** Thông báo nhỏ, tạm thời xuất hiện ở góc màn hình.
-   **Sử dụng:** Thông báo nhanh về các hành động (ví dụ: "Đăng nhập thành công!").
-   **Nguồn:** Shadcn UI.

## 5. Component đặc thù ứng dụng

### 5.1. QuestionDisplay
-   **Mô tả:** Component hiển thị nội dung của một câu hỏi, có thể bao gồm văn bản hoặc hình ảnh.
-   **Sử dụng:** Trang làm Quiz.
-   **Nguồn:** Custom.

### 5.2. AnswerOptions
-   **Mô tả:** Component hiển thị các lựa chọn đáp án cho một câu hỏi, có thể là RadioGroup, Checkbox hoặc Input tùy thuộc vào loại câu hỏi.
-   **Sử dụng:** Trang làm Quiz.
-   **Nguồn:** Custom (kết hợp Shadcn UI form components).

---

Tài liệu này sẽ là cơ sở để phát triển các component frontend, đảm bảo tính module hóa và khả năng bảo trì của codebase.