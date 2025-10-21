# csquizz: Nền tảng Test Online về Computer Science

## 1. Tổng quan & Ý tưởng cốt lõi (Project Overview & Core Concept)

**csquizz** là một trang web cung cấp các bài test trực tuyến về Khoa học Máy tính (Computer Science).

-   **Mục tiêu:** Giúp sinh viên, người học, và bất kỳ ai quan tâm đến lĩnh vực Khoa học máy tính có thể luyện tập kiến thức, kiểm tra trình độ và theo dõi sự tiến bộ của bản thân.
-   **Ý tưởng cốt lõi:**
    -   Người dùng truy cập web, chọn chủ đề, làm bài test và nhận kết quả ngay lập tức.
    -   Kết quả được lưu lại (đối với người dùng có tài khoản) để theo dõi quá trình học tập.
    -   Admin có quyền quản lý ngân hàng câu hỏi để mở rộng và cập nhật nội dung.

## 2. Tính năng chính (Main Features)

### Dành cho Người dùng (End-user)

-   Làm quiz online theo từng chủ đề (Data Structures, Algorithms, Databases, OS, Networking, AI,...).
-   Xem kết quả ngay sau khi nộp bài (điểm số, đáp án đúng/sai, giải thích chi tiết).
-   Đăng ký / Đăng nhập để lưu lại lịch sử làm bài.
-   Tìm kiếm và lọc bài quiz theo độ khó hoặc chủ đề.

### Dành cho Quản trị viên (Admin)

-   Thêm, sửa, xóa các bài quiz.
-   Quản lý ngân hàng câu hỏi.

## 3. Đặc tả Kỹ thuật (Technical Specifications)

### Công nghệ sử dụng

-   **Frontend:**
    -   **Framework:** Next.js (App Router)
    -   **Ngôn ngữ:** TypeScript, React Server Components
    -   **Styling:** Tailwind CSS, shadcn/ui
-   **Backend:** Rust (sử dụng framework Axum)
-   **Database:** PostgreSQL
-   **Authentication:** JWT / Session-based
-   **Triển khai (Deployment):** Docker, và các dịch vụ cloud (AWS, Heroku, Vercel).

### Cấu trúc dự án

<details>
<summary>Cấu trúc Backend</summary>

```
backend/
├── src/
│   ├── main.rs                # Entry point
│   ├── routes/                # Định nghĩa API routes
│   │   ├── quiz.rs
│   │   ├── auth.rs
│   │   └── admin.rs
│   ├── models/                # Định nghĩa data structures (Quiz, User, Result)
│   ├── db/                    # Tương tác với database
│   ├── services/              # Business logic
│   └── utils/                 # Helper functions (auth, validation)
└── Cargo.toml
```

</details>

<details>
<summary>Cấu trúc Frontend</summary>

```
frontend/
├── app/
│   ├── page.tsx               # Trang chủ (danh sách quiz)
│   ├── quiz/[id]/page.tsx     # Trang làm quiz
│   ├── result/[id]/page.tsx   # Trang kết quả
│   ├── profile/page.tsx       # Trang hồ sơ cá nhân
│   └── admin/page.tsx         # Trang quản trị
├── components/                # UI Components (Button, Card, Navbar,...)
├── lib/                       # Helper functions (API client, auth)
├── styles/                    # Global styles
└── types/                     # TypeScript type definitions
```

</details>

### API Endpoints

-   `POST /auth/login`: Đăng nhập
-   `POST /auth/register`: Đăng ký
-   `GET /quiz/:id`: Lấy thông tin chi tiết một quiz
-   `POST /quiz/:id/submit`: Nộp bài và chấm điểm
-   `GET /user/history`: Xem lịch sử làm bài của người dùng
-   `POST /admin/quiz`: (Admin) Thêm quiz mới
-   `PUT /admin/quiz/:id`: (Admin) Cập nhật quiz
-   `DELETE /admin/quiz/:id`: (Admin) Xóa quiz

## 4. Cài đặt & Chạy dự án (Setup & Run)

*(Phần này cần được bổ sung hướng dẫn chi tiết)*

### Yêu cầu

-   Node.js
-   Rust
-   PostgreSQL
-   Docker (tùy chọn)

### Hướng dẫn cài đặt

```bash
# Clone the repository
git clone <repository-url>
cd csquizz

# Cài đặt dependencies cho backend
cd backend
cargo build

# Cài đặt dependencies cho frontend
cd ../frontend
npm install
```

### Chạy dự án

```bash
# Chạy backend server
cd backend
cargo run

# Chạy frontend dev server
cd ../frontend
npm run dev
```

---
*Tài liệu này được tạo tự động và có thể cần được bổ sung.*
