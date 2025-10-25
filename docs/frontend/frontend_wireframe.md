# Wireframe cho Front-end - Quiz-Bank

Tài liệu này mô tả cấu trúc wireframe cho giao diện người dùng của ứng dụng Quiz-Bank. Mỗi trang được định nghĩa với các thành phần chính và bố cục cơ bản.

## 1. Các thành phần chung (Global Components)

### 1.1. Thanh điều hướng (Navbar)
- **Vị trí:** Trên cùng của tất cả các trang.
- **Thành phần:**
  - **Bên trái:** Logo / Tên ứng dụng ("Quiz-Bank").
  - **Bên phải (chưa đăng nhập):**
    - Nút "Đăng nhập" (Login).
    - Nút "Đăng ký" (Sign Up).
  - **Bên phải (đã đăng nhập):**
    - Tên người dùng.
    - Menu thả xuống (Dropdown) với các tùy chọn:
      - "Hồ sơ của tôi" (My Profile) -> điều hướng đến `/profile`.
      - "Đăng xuất" (Logout).

### 1.2. Chân trang (Footer)
- **Vị trí:** Dưới cùng của tất cả các trang.
- **Thành phần:**
  - Copyright © 2025 Quiz-Bank.
  - Liên kết đến các trang giới thiệu hoặc mạng xã hội (nếu có).

---

## 2. Wireframe cho từng trang

### 2.1. Trang chủ (Home Page)
- **URL:** `/`
- **File:** `app/page.tsx`
- **Mô tả:** Hiển thị danh sách các chủ đề quiz có sẵn để người dùng lựa chọn.
- **Bố cục:**
  - **Header:** Navbar chung.
  - **Main Content:**
    - **Phần trên:**
      - Tiêu đề lớn: "Luyện tập kiến thức Khoa học máy tính".
      - Thanh tìm kiếm để lọc các chủ đề quiz.
    - **Phần dưới:**
      - Lưới (Grid) các thẻ (Card) chủ đề quiz.
      - Mỗi thẻ `QuizItem` bao gồm:
        - Tên chủ đề (ví dụ: "Data Structures", "Algorithms").
        - Mô tả ngắn.
        - Nút "Bắt đầu" (Start) -> điều hướng đến `/quiz/[id]`.
      - Phân trang (Pagination) nếu có nhiều chủ đề. (6 chủ đề mỗi trang)
  - **Footer:** Footer chung.

### 2.2. Trang làm Quiz (Quiz Page)
- **URL:** `/quiz/[id]`
- **File:** `app/quiz/[id]/page.tsx`
- **Mô tả:** Giao diện làm bài trắc nghiệm.
- **Bố cục:**
  - **Header:** Navbar chung.
  - **Main Content:**
    - **Thanh tiến độ (Progress Bar):** Hiển thị `%` số câu đã hoàn thành.
    - **Khu vực câu hỏi:**
      - Hiển thị nội dung câu hỏi (văn bản hoặc hình ảnh).
    - **Khu vực đáp án:**
      - Danh sách các lựa chọn (A, B, C, D) dạng radio button cho single-choice question
      - Hoặc checkbox cho multiple-choice question.
      - Hoặc trường nhập văn bản cho text answer question.
    - **Thanh điều hướng câu hỏi:**
      - Nút "Câu trước" (Previous).
      - Nút "Câu tiếp theo" (Next).
      - Nút "Nộp bài" (Submit) -> hiển thị khi ở câu hỏi cuối cùng hoặc sau khi người dùng xác nhận.
  - **Footer:** Footer chung.

### 2.3. Trang kết quả (Result Page)
- **URL:** `/result/[id]`
- **File:** `app/result/[id]/page.tsx`
- **Mô tả:** Hiển thị kết quả chi tiết sau khi người dùng nộp bài.
- **Bố cục:**
  - **Header:** Navbar chung.
  - **Main Content:**
    - **Phần tổng kết:**
      - Điểm số: "Bạn đạt 8/10 câu đúng".
      - Thông điệp chúc mừng hoặc khuyến khích.
    - **Phần chi tiết:**
      - Danh sách các câu hỏi đã làm.
      - Mỗi mục câu hỏi hiển thị:
        - Nội dung câu hỏi.
        - Đáp án bạn chọn (highlight màu xanh nếu đúng, màu đỏ nếu sai).
        - Đáp án đúng (hiển thị nếu bạn chọn sai).
        - Giải thích chi tiết cho đáp án.
    - **Nút hành động:**
      - Nút "Làm lại Quiz" (Retake Quiz).
      - Nút "Quay về trang chủ" (Back to Home).
  - **Footer:** Footer chung.

### 2.4. Trang hồ sơ cá nhân (User Profile Page)
- **URL:** `/profile`
- **File:** `app/profile/page.tsx`
- **Mô tả:** Hiển thị thông tin và lịch sử làm bài của người dùng. Yêu cầu đăng nhập.
- **Bố cục:**
  - **Header:** Navbar chung.
  - **Main Content:**
    - **Thông tin người dùng:**
      - Avatar, Tên, Email.
    - **Thống kê tổng quan:**
      - Điểm trung bình.
      - Tổng số bài quiz đã làm.
    - **Lịch sử làm bài:**
      - Bảng (Table) hiển thị lịch sử các bài đã làm.
      - Các cột: Tên Quiz, Ngày làm, Điểm số, Nút "Xem lại" -> điều hướng đến `/result/[id]`.
  - **Footer:** Footer chung.

### 2.5. Trang quản trị (Admin Page)
- **URL:** `/admin`
- **File:** `app/admin/page.tsx`
- **Mô tả:** Giao diện cho quản trị viên quản lý ngân hàng câu hỏi. Yêu cầu đăng nhập với vai trò admin.
- **Bố cục:**
  - **Header:** Navbar chung.
  - **Main Content:**
    - Tiêu đề: "Quản lý ngân hàng câu hỏi".
    - Nút "Thêm câu hỏi mới" (Add New Question).
    - Bảng (Table) danh sách các câu hỏi/quiz.
    - Các cột: ID, Tên Quiz/Câu hỏi, Chủ đề, Độ khó, Hành động (Sửa, Xóa).
  - **Footer:** Footer chung.

### 2.6. Trang Đăng nhập / Đăng ký (Auth Pages)
- **URL:** `/login`, `/register`
- **Mô tả:** Form cho phép người dùng đăng nhập hoặc tạo tài khoản mới.
- **Bố cục:**
  - Form đơn giản ở giữa trang.
  - **Trang Đăng nhập:**
    - Tiêu đề "Đăng nhập".
    - Trường nhập Email.
    - Trường nhập Mật khẩu.
    - Nút "Đăng nhập".
    - Liên kết: "Chưa có tài khoản? Đăng ký ngay".
  - **Trang Đăng ký:**
    - Tiêu đề "Đăng ký".
    - Trường nhập Tên.
    - Trường nhập Email.
    - Trường nhập Mật khẩu.
    - Nút "Đăng ký".
    - Liên kết: "Đã có tài khoản? Đăng nhập".
