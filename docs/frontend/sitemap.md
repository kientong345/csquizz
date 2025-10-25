# Sitemap cho ứng dụng Quiz-Bank

Tài liệu này cung cấp cái nhìn tổng quan về cấu trúc và hệ thống phân cấp của các trang trong ứng dụng Quiz-Bank, giúp hình dung luồng điều hướng của người dùng.

## 1. Cấu trúc trang chính

-   **Trang chủ (Home Page)**
    -   **URL:** `/`
    -   **Mô tả:** Điểm truy cập chính của ứng dụng, hiển thị danh sách các chủ đề quiz và thanh tìm kiếm.
    -   **Truy cập từ:** Mọi nơi (Navbar, sau khi làm quiz).
    -   **Điều hướng đến:** Trang làm Quiz, Trang Đăng nhập/Đăng ký, Trang Hồ sơ (nếu đã đăng nhập).

-   **Trang làm Quiz (Quiz Page)**
    -   **URL:** `/quiz/[id]` (ví dụ: `/quiz/data-structures`)
    -   **Mô tả:** Hiển thị các câu hỏi trắc nghiệm, cho phép người dùng chọn đáp án và điều hướng giữa các câu hỏi.
    -   **Truy cập từ:** Trang chủ (khi chọn một chủ đề quiz).
    -   **Điều hướng đến:** Trang Kết quả (sau khi nộp bài).

-   **Trang Kết quả (Result Page)**
    -   **URL:** `/result/[id]` (ví dụ: `/result/12345`)
    -   **Mô tả:** Hiển thị điểm số, đáp án chi tiết và giải thích sau khi người dùng hoàn thành quiz.
    -   **Truy cập từ:** Trang làm Quiz (sau khi nộp bài), Trang Hồ sơ (xem lại lịch sử).
    -   **Điều hướng đến:** Trang chủ, Làm lại Quiz.

-   **Trang Hồ sơ cá nhân (User Profile Page)**
    -   **URL:** `/profile`
    -   **Mô tả:** Hiển thị thông tin cá nhân, lịch sử làm bài, và các thống kê liên quan của người dùng.
    -   **Truy cập từ:** Navbar (sau khi đăng nhập).
    -   **Yêu cầu:** Đăng nhập.
    -   **Điều hướng đến:** Trang Kết quả (xem lại lịch sử).

-   **Trang Quản trị (Admin Page)**
    -   **URL:** `/admin`
    -   **Mô tả:** Giao diện quản lý ngân hàng câu hỏi (thêm, sửa, xóa quiz/câu hỏi).
    -   **Truy cập từ:** Navbar (chỉ hiển thị cho Admin).
    -   **Yêu cầu:** Đăng nhập với vai trò Admin.

-   **Trang Đăng nhập (Login Page)**
    -   **URL:** `/login`
    -   **Mô tả:** Form cho phép người dùng đăng nhập vào hệ thống.
    -   **Truy cập từ:** Navbar (khi chưa đăng nhập), Trang Đăng ký.
    -   **Điều hướng đến:** Trang chủ, Trang Đăng ký.

-   **Trang Đăng ký (Register Page)**
    -   **URL:** `/register`
    -   **Mô tả:** Form cho phép người dùng tạo tài khoản mới.
    -   **Truy cập từ:** Navbar (khi chưa đăng nhập), Trang Đăng nhập.
    -   **Điều hướng đến:** Trang Đăng nhập.

## 2. Luồng người dùng cơ bản

1.  **Người dùng chưa đăng nhập:**
    -   Truy cập `/` (Trang chủ) -> Xem danh sách quiz.
    -   Chọn quiz -> `/quiz/[id]` (Làm quiz).
    -   Nộp bài -> `/result/[id]` (Xem kết quả).
    -   Có thể chọn Đăng nhập/Đăng ký từ Navbar hoặc từ trang kết quả để lưu lịch sử.

2.  **Người dùng đã đăng nhập:**
    -   Truy cập `/` (Trang chủ) -> Xem danh sách quiz.
    -   Chọn quiz -> `/quiz/[id]` (Làm quiz).
    -   Nộp bài -> `/result/[id]` (Xem kết quả, kết quả được lưu vào hồ sơ).
    -   Truy cập `/profile` (Hồ sơ cá nhân) -> Xem lịch sử, thông tin.

3.  **Quản trị viên:**
    -   Đăng nhập với tài khoản Admin.
    -   Truy cập `/admin` (Trang Quản trị) -> Quản lý dữ liệu.

---

Tài liệu này cung cấp một cái nhìn tổng thể về cấu trúc điều hướng của ứng dụng Quiz-Bank, làm cơ sở cho việc phát triển và kiểm thử.