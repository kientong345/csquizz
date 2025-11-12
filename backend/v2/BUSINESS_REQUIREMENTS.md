# Tài liệu Yêu cầu Nghiệp vụ (BRD) cho Dự án csquizz

-   **Tên dự án:** csquizz - Nền tảng Test Online về Computer Science
-   **Ngày tạo:** 2023-10-27
-   **Phiên bản:** 1.0

## 1. Giới thiệu

### 1.1. Bối cảnh dự án

Lĩnh vực Khoa học Máy tính (Computer Science) đang phát triển nhanh chóng, đòi hỏi người học phải liên tục cập nhật và củng cố kiến thức. Tuy nhiên, việc tìm kiếm một nền tảng tập trung, chất lượng để thực hành và kiểm tra kiến thức vẫn còn hạn chế.

### 1.2. Mục tiêu dự án

**csquizz** được xây dựng để trở thành một trang web chuyên cung cấp các bài test trắc nghiệm trực tuyến về Khoa học Máy tính. Mục tiêu chính bao gồm:

-   **For Users:** Cung cấp một công cụ học tập hiệu quả, giúp người dùng ôn luyện, tự đánh giá năng lực và theo dõi sự tiến bộ của bản thân.
-   **For Admins:** Xây dựng một hệ thống quản lý nội dung mạnh mẽ, cho phép dễ dàng mở rộng và cập nhật ngân hàng câu hỏi.
-   **For Business:** Tạo ra một nền tảng uy tín, thu hút cộng đồng người học và các chuyên gia trong ngành.

### 1.3. Phạm vi tài liệu

Tài liệu này mô tả các yêu cầu nghiệp vụ ở mức cao, bao gồm các tính năng chính, đối tượng sử dụng và các yêu cầu phi chức năng của hệ thống.

## 2. Đối tượng sử dụng (Stakeholders)

| Vai trò | Mô tả |
| :--- | :--- |
| **Người dùng (User)** | Sinh viên, lập trình viên, người tự học muốn kiểm tra và củng cố kiến thức. |
| **Quản trị viên (Admin)** | Người chịu trách nhiệm quản lý nội dung, người dùng và đảm bảo chất lượng của nền tảng. |

## 3. Yêu cầu Chức năng (Functional Requirements)

### FR1: Quản lý Người dùng & Xác thực

| ID | Yêu cầu | Mô tả chi tiết |
| :--- | :--- | :--- |
| **FR1.1** | Đăng ký tài khoản | Người dùng có thể tạo tài khoản mới bằng cách cung cấp tên người dùng, email và mật khẩu. Hệ thống cần xác thực email là duy nhất. |
| **FR1.2** | Đăng nhập | Người dùng có thể đăng nhập vào hệ thống bằng email và mật khẩu. |
| **FR1.3** | Tích hợp Google Login | Cho phép người dùng đăng ký/đăng nhập nhanh chóng thông qua tài khoản Google. |
| **FR1.4** | Phân quyền | Hệ thống phải có hai vai trò: `USER` (mặc định) và `ADMIN`. Admin có quyền truy cập các chức năng quản trị. |
| **FR1.5** | Đăng xuất | Người dùng có thể kết thúc phiên đăng nhập một cách an toàn. |
| **FR1.6** | Quản lý mật khẩu | Người dùng có thể yêu cầu đặt lại mật khẩu nếu quên và thay đổi mật khẩu sau khi đăng nhập. |

### FR2: Quản lý Nội dung (Admin)

| ID | Yêu cầu | Mô tả chi tiết |
| :--- | :--- | :--- |
| **FR2.1** | Quản lý Chủ đề (Category) | Admin có thể thực hiện các thao tác Thêm, Sửa, Xóa các chủ đề (ví dụ: "Thuật toán", "Mạng máy tính"). |
| **FR2.2** | Quản lý Bài Quiz | Admin có thể Thêm, Sửa, Xóa các bài quiz, bao gồm thiết lập tiêu đề, mô tả, độ khó và gán vào một chủ đề. |
| **FR2.3** | Quản lý Câu hỏi (Question) | Admin có thể thêm/sửa/xóa câu hỏi trong một quiz, hỗ trợ các loại: trắc nghiệm một lựa chọn, nhiều lựa chọn và điền từ. |
| **FR2.4** | Thiết lập Đáp án | Đối với mỗi câu hỏi, Admin phải chỉ định được đáp án đúng và cung cấp phần giải thích chi tiết. |
| **FR2.5** | Quản lý Người dùng | Admin có thể xem danh sách người dùng và thay đổi vai trò của họ. |

### FR3: Trải nghiệm Làm Quiz (User)

| ID | Yêu cầu | Mô tả chi tiết |
| :--- | :--- | :--- |
| **FR3.1** | Duyệt và Lọc Quiz | Người dùng có thể xem danh sách các bài quiz theo chủ đề và lọc theo độ khó (Dễ, Trung bình, Khó). |
| **FR3.2** | Bắt đầu Quiz | Người dùng có thể bắt đầu làm một bài quiz. Giao diện làm bài cần hiển thị rõ ràng câu hỏi và các lựa chọn. |
| **FR3.3** | Nộp bài | Sau khi hoàn thành, người dùng có thể nộp bài để chấm điểm. |
| **FR3.4** | Xem kết quả tức thì | Hệ thống phải chấm điểm tự động và trả về kết quả ngay lập tức, bao gồm điểm số và tổng số câu đúng. |
| **FR3.5** | Xem lại bài làm | Người dùng có thể xem lại chi tiết bài làm, thấy được câu nào đúng/sai, đáp án chính xác và phần giải thích. |

### FR4: Tương tác & Cộng đồng

| ID | Yêu cầu | Mô tả chi tiết |
| :--- | :--- | :--- |
| **FR4.1** | Thích Quiz | Người dùng có thể "like" một bài quiz để thể hiện sự yêu thích. |
| **FR4.2** | Bình luận | Người dùng có thể viết bình luận, thảo luận hoặc đặt câu hỏi ở cuối mỗi bài quiz. |
| **FR4.3** | Tương tác với bình luận | Người dùng có thể "like" các bình luận hữu ích của người khác. |

### FR5: Hồ sơ cá nhân & Lịch sử

| ID | Yêu cầu | Mô tả chi tiết |
| :--- | :--- | :--- |
| **FR5.1** | Trang cá nhân | Người dùng có một trang cá nhân để xem và cập nhật thông tin cơ bản (tên hiển thị, ảnh đại diện). |
| **FR5.2** | Lịch sử làm bài | Hệ thống phải lưu lại lịch sử các bài quiz người dùng đã làm, bao gồm điểm số và ngày làm. |
| **FR5.3** | Xem lại kết quả cũ | Từ trang lịch sử, người dùng có thể nhấp để xem lại chi tiết kết quả của một lần làm bài trước đó. |

## 4. Yêu cầu Phi chức năng (Non-Functional Requirements)

| ID | Yêu cầu | Mô tả |
| :--- | :--- | :--- |
| **NFR1** | Hiệu năng (Performance) | - Thời gian tải trang không quá 3 giây.<br>- Thời gian phản hồi của API cho các tác vụ thông thường (lấy danh sách quiz, nộp bài) dưới 500ms. |
| **NFR2** | Trải nghiệm người dùng (UI/UX) | - Giao diện phải sạch sẽ, hiện đại, và thân thiện.<br>- Tương thích và hiển thị tốt trên các thiết bị phổ biến (desktop, tablet, mobile). |
| **NFR3** | Bảo mật (Security) | - Mật khẩu người dùng phải được băm (hash) an toàn.<br>- Dữ liệu truyền tải giữa client và server phải được mã hóa (HTTPS).<br>- Hệ thống phải có cơ chế chống lại các tấn công phổ biến (SQL Injection, XSS). |
| **NFR4** | Khả năng mở rộng (Scalability) | - Kiến trúc hệ thống phải cho phép dễ dàng thêm các tính năng mới.<br>- Có khả năng chịu tải khi lượng người dùng đồng thời tăng lên. |

## 5. Giả định và Ràng buộc

-   **Giả định:**
    -   Người dùng có kiến thức cơ bản về việc sử dụng web.
    -   Nội dung ban đầu cho các bài quiz sẽ do đội ngũ dự án chuẩn bị.
-   **Ràng buộc:**
    -   Dự án sẽ được phát triển bằng các công nghệ đã chọn: Rust (Backend) và Next.js (Frontend).
    -   Ngân sách và thời gian phát triển có giới hạn (cần xác định cụ thể).

---