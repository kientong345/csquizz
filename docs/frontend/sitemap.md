# Sitemap cho ứng dụng Quiz-Bank

Tài liệu này cung cấp cái nhìn tổng quan về cấu trúc và hệ thống phân cấp của các trang trong ứng dụng Quiz-Bank, giúp hình dung luồng điều hướng của người dùng.

## 1. Cấu trúc trang chính

-   **Trang chủ (Home Page)**
    -   **URL:** `/`
    -   **Mô tả:** Điểm truy cập chính, hiển thị danh sách các **chủ đề** quiz (categories).
    -   **Truy cập từ:** Mọi nơi (Navbar, sau khi làm quiz).
    -   **Điều hướng đến:** Trang Danh sách Quiz, Trang Đăng nhập/Đăng ký.

-   **Trang Danh sách Quiz (Quiz List Page)**
    -   **URL:** `/category/[id]` (ví dụ: `/category/data-structures`)
    -   **Mô tả:** Hiển thị danh sách các **bài quiz** thuộc một chủ đề cụ thể.
    -   **Truy cập từ:** Trang chủ (khi chọn một chủ đề).
    -   **Điều hướng đến:** Trang làm Quiz.

-   **Trang làm Quiz (Quiz Page)**
    -   **URL:** `/quiz/[id]` (ví dụ: `/quiz/123`)
    -   **Mô tả:** Hiển thị các câu hỏi trắc nghiệm, cho phép người dùng làm bài.
    -   **Truy cập từ:** Trang Danh sách Quiz (khi chọn một bài quiz).
    -   **Điều hướng đến:** Trang Kết quả (sau khi nộp bài).

-   **Trang Kết quả (Result Page)**
    -   **URL:** `/result/[id]` (ví dụ: `/result/5678`)
    -   **Mô tả:** Hiển thị điểm số và chi tiết kết quả sau khi hoàn thành quiz.
    -   **Truy cập từ:** Trang làm Quiz, Trang Hồ sơ.
    -   **Điều hướng đến:** Trang chủ, Làm lại Quiz.

-   **Trang Hồ sơ cá nhân (User Profile Page)**
    -   **URL:** `/profile`
    -   **Mô tả:** Hiển thị thông tin cá nhân và lịch sử làm bài của người dùng.
    -   **Truy cập từ:** Navbar (sau khi đăng nhập).
    -   **Yêu cầu:** Đăng nhập.
    -   **Điều hướng đến:** Trang Kết quả (xem lại lịch sử).

-   **Trang Quản trị (Admin Page)**
    -   **URL:** `/admin`
    -   **Mô tả:** Giao diện quản lý ngân hàng câu hỏi.
    -   **Truy cập từ:** Navbar (chỉ hiển thị cho Admin).
    -   **Yêu cầu:** Đăng nhập với vai trò Admin.

-   **Trang Đăng nhập (Login Page)**
    -   **URL:** `/login`
    -   **Mô tả:** Form cho phép người dùng đăng nhập.
    -   **Truy cập từ:** Navbar, Trang Đăng ký.
    -   **Điều hướng đến:** Trang chủ, Trang Đăng ký.

-   **Trang Đăng ký (Register Page)**
    -   **URL:** `/register`
    -   **Mô tả:** Form cho phép người dùng tạo tài khoản mới.
    -   **Truy cập từ:** Navbar, Trang Đăng nhập.
    -   **Điều hướng đến:** Trang Đăng nhập.

## 2. Luồng người dùng cơ bản

1.  **Người dùng chưa đăng nhập:**
    -   Truy cập `/` (Trang chủ) -> Xem danh sách **chủ đề**.
    -   Chọn chủ đề -> `/category/[id]` (Trang danh sách quiz) -> Xem danh sách **bài quiz**.
    -   Chọn bài quiz -> `/quiz/[id]` (Làm quiz).
    -   Nộp bài -> `/result/[id]` (Xem kết quả).

2.  **Người dùng đã đăng nhập:**
    -   (Luồng tương tự như trên, nhưng kết quả sẽ được lưu vào hồ sơ).
    -   Truy cập `/profile` (Hồ sơ cá nhân) -> Xem lịch sử, thông tin.

3.  **Quản trị viên:**
    -   Đăng nhập với tài khoản Admin.
    -   Truy cập `/admin` (Trang Quản trị) -> Quản lý dữ liệu.