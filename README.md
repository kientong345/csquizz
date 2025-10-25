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

## 3. Đặc tả Kỹ thuật & Thiết kế (Technical Specifications & Design)

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
├── Cargo.toml                 # Cấu hình dự án Rust
├── config.json                # Cấu hình ứng dụng
├── migrations/                # Các script di chuyển database
├── schema/                    # Định nghĩa schema database (Prisma)
└── src/                       # Mã nguồn chính của backend
    ├── app.rs                 # Định nghĩa ứng dụng Axum
    ├── main.rs                # Điểm khởi chạy ứng dụng
    ├── config/                # Cấu hình ứng dụng
    ├── controller/            # Xử lý logic nghiệp vụ cho các route
    ├── database/              # Kết nối và thao tác database
    ├── middleware/            # Các middleware xử lý request
    ├── models/                # Định nghĩa các struct dữ liệu (Quiz, User, Result, v.v.)
    ├── routes/                # Định nghĩa các API endpoint
    └── utils/                 # Các hàm tiện ích
```

</details>

<details>
<summary>Cấu trúc Frontend</summary>

```
frontend/
└── csquizz-web-app/           # Thư mục gốc của ứng dụng Next.js
    ├── public/                # Các tài nguyên tĩnh (ảnh, icon)
    ├── app/                   # Các trang và layout của ứng dụng (App Router)
    ├── components/            # Các UI component có thể tái sử dụng
    ├── lib/                   # Các hàm tiện ích và client API
    ├── styles/                # Các file CSS toàn cục
    ├── types/                 # Định nghĩa kiểu dữ liệu TypeScript
    ├── package.json           # Cấu hình dự án Node.js/npm
    ├── next.config.ts         # Cấu hình Next.js
    ├── tsconfig.json          # Cấu hình TypeScript
    └── ...                    # Các file cấu hình và thư mục khác
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

### Thiết kế & Tài liệu

Các tài liệu thiết kế và cấu trúc ứng dụng được tạo ra để hỗ trợ quá trình phát triển:

-   **Wireframe cho Front-end:** `frontend_wireframe.md`
    -   Mô tả bố cục và cấu trúc cơ bản của từng trang.
    -   Kèm theo các file SVG minh họa (`*.svg`) cho từng trang để hình dung trực quan.
-   **Thiết kế UI (Mockup) cho Front-end:** `frontend_ui_design.md`
    -   Đặc tả chi tiết về bảng màu, typography, khoảng cách, và phong cách của các thành phần UI dựa trên Shadcn UI và Tailwind CSS.
-   **Sitemap cho ứng dụng:** `sitemap.md`
    -   Phác thảo cấu trúc và hệ thống phân cấp của các trang, giúp hình dung luồng điều hướng.

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