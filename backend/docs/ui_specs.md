# Đặc tả Giao diện & Trải nghiệm người dùng (UI/UX)

## 1. Trang Chủ (Homepage)

-   **Mục đích:** Giới thiệu trang web, hiển thị danh sách các bài quiz và cung cấp công cụ tìm kiếm.
-   **Các thành phần chính (Key Components):**
    -   **Header (Thanh đầu trang):**
        -   *Bên trái:* Logo và Tên trang web ("Quiz-Bank").
        -   *Bên phải (chưa đăng nhập):* Nút "Đăng nhập", "Đăng ký".
        -   *Bên phải (đã đăng nhập):* Avatar người dùng, tên người dùng, và menu dropdown có link tới "Hồ sơ" và "Đăng xuất".
    -   **Khu vực chính (Hero Section):**
        -   Tiêu đề lớn, hấp dẫn (ví dụ: "Thử thách kiến thức Khoa học Máy tính của bạn").
        -   Một đoạn mô tả ngắn về trang web.
        -   Thanh tìm kiếm (Search Bar) nổi bật, cho phép tìm theo tên quiz.
    -   **Danh sách Quiz (Quiz List):**
        -   Tiêu đề khu vực (ví dụ: "Các chủ đề nổi bật").
        -   Một lưới (grid) các Thẻ Quiz (Quiz Card).
        -   Mỗi Thẻ Quiz cần có:
            -   Tên quiz (ví dụ: "Algorithms", "Databases").
            -   Mô tả ngắn gọn.
            -   Thông tin phụ: Số câu hỏi, độ khó (Dễ/Trung bình/Khó).
            -   Nút kêu gọi hành động: "Bắt đầu".
-   **Luồng tương tác:**
    1.  Người dùng truy cập trang, lướt xem các quiz.
    2.  Sử dụng thanh tìm kiếm để lọc danh sách.
    3.  Nhấp vào nút "Bắt đầu" trên một thẻ quiz để được chuyển đến Trang Làm Quiz.

---

## 2. Trang Làm Quiz (Quiz Page)

-   **Mục đích:** Hiển thị câu hỏi và các lựa chọn, cho phép người dùng trả lời và nộp bài.
-   **Các thành phần chính:**
    -   **Header:**
        -   Tên của bài quiz đang làm.
        -   Thanh tiến trình (Progress Bar) hiển thị % hoàn thành hoặc số câu (ví dụ: "Câu 5 / 20").
    -   **Khu vực câu hỏi (Question Area):**
        -   Nội dung câu hỏi: `<h2>` hoặc `<h3>` "Câu 5: [Nội dung câu hỏi...]".
        -   Danh sách các lựa chọn: Dạng radio button (cho câu hỏi chọn 1 đáp án) hoặc checkbox (cho câu hỏi chọn nhiều đáp án).
    -   **Thanh điều hướng (Navigation Footer):**
        -   Nút "Câu trước" (vô hiệu hóa ở câu đầu tiên).
        -   Nút "Câu tiếp theo".
        -   Ở câu hỏi cuối cùng, nút "Câu tiếp theo" được thay bằng nút "Nộp bài".
-   **Luồng tương tác:**
    1.  Người dùng chọn một đáp án.
    2.  Nhấp "Câu tiếp theo" để chuyển sang câu hỏi mới (lựa chọn của câu cũ được lưu lại).
    3.  Có thể dùng "Câu trước" để quay lại sửa đáp án.
    4.  Sau khi nhấp "Nộp bài", một hộp thoại xác nhận hiện ra ("Bạn chắc chắn muốn kết thúc bài làm?").
    5.  Xác nhận sẽ chuyển người dùng đến Trang Kết Quả.

---

## 3. Trang Kết Quả (Results Page)

-   **Mục đích:** Cung cấp phản hồi tức thì về hiệu suất làm bài của người dùng.
-   **Các thành phần chính:**
    -   **Khu vực tổng kết (Summary Section):**
        -   Tiêu đề: "Kết quả bài quiz: [Tên quiz]".
        -   Điểm số: Hiển thị nổi bật, rõ ràng (ví dụ: "8/10" hoặc "80%").
        -   Số câu đúng, số câu sai.
        -   Nút hành động: "Làm lại", "Quay về trang chủ".
        -   (Nếu chưa đăng nhập) Một thông điệp mời đăng ký để lưu lại kết quả này.
    -   **Khu vực xem lại chi tiết (Detailed Review):**
        -   Danh sách lặp lại tất cả các câu hỏi trong bài quiz.
        -   Mỗi mục câu hỏi hiển thị:
            -   Nội dung câu hỏi.
            -   Đáp án người dùng đã chọn (ví dụ: highlight màu xám).
            -   Đáp án đúng (highlight màu xanh lá). Nếu người dùng chọn sai, đáp án của họ sẽ highlight màu đỏ.
            -   Một khu vực nhỏ bên dưới để hiển thị giải thích cho đáp án đúng.
-   **Luồng tương tác:**
    1.  Người dùng xem điểm số tổng quan.
    2.  Kéo xuống để xem lại chi tiết từng câu trả lời của mình.

---

## 4. Trang Hồ Sơ & Lịch Sử (Profile & History Page)

-   **Mục đích:** Cho phép người dùng đã đăng nhập xem lại lịch sử và theo dõi tiến trình.
-   **Các thành phần chính:**
    -   **Thông tin người dùng:** Avatar, tên, email.
    -   **Khu vực lịch sử:**
        -   Tiêu đề: "Lịch sử làm bài".
        -   Một bảng (table) liệt kê các lần làm bài trước đây.
        -   Các cột của bảng: Tên Quiz, Điểm số, Ngày làm, Nút "Xem lại".
-   **Luồng tương tác:**
    -   Người dùng nhấp vào nút "Xem lại" trên một hàng để được chuyển đến Trang Kết Quả chi tiết của lần làm bài đó.

---

## 5. Trang Quản Trị (Admin Dashboard)

-   **Mục đích:** Cung cấp giao diện CRUD (Tạo, Đọc, Cập nhật, Xóa) cho admin.
-   **Các thành phần chính:**
    -   **Giao diện chính (CRUD Quizzes):**
        -   Tiêu đề: "Quản lý Quiz".
        -   Nút `+ Thêm Quiz Mới`.
        -   Một bảng liệt kê tất cả các quiz.
        -   Các cột của bảng: Tên Quiz, Chủ đề, Số câu hỏi, Ngày tạo, và cột Hành động (chứa nút "Sửa" và "Xóa").
    -   **Form Thêm/Sửa Quiz (hiển thị dạng trang riêng hoặc modal):**
        -   Các trường nhập liệu: Tiêu đề, Mô tả, Chủ đề, Độ khó.
        -   Nút "Lưu" hoặc "Hủy".
        -   Trong trang Sửa Quiz, có thêm khu vực "Quản lý Câu hỏi":
            -   Danh sách các câu hỏi hiện tại của quiz đó (mỗi câu có nút Sửa/Xóa).
            -   Nút `+ Thêm câu hỏi`.
-   **Luồng tương tác:**
    1.  Admin nhấp "Sửa" trên một quiz để vào trang chỉnh sửa chi tiết quiz đó và các câu hỏi bên trong.
    2.  Admin nhấp "Xóa" sẽ có hộp thoại xác nhận.
