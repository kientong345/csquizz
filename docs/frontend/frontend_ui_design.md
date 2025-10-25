# Thiết kế UI (Mockup) cho Front-end - Quiz-Bank

Tài liệu này mô tả các yếu tố thiết kế giao diện người dùng (UI) cho ứng dụng Quiz-Bank, tập trung vào việc sử dụng Shadcn UI và Tailwind CSS để tạo ra một giao diện hiện đại, sạch sẽ và phản hồi tốt.

## 1. Nguyên tắc thiết kế chung

-   **Phong cách:** Tối giản, hiện đại, sạch sẽ, tập trung vào trải nghiệm người dùng.
-   **Responsive:** Giao diện sẽ được thiết kế để hiển thị tốt trên mọi kích thước màn hình (desktop, tablet, mobile).
-   **Khả năng truy cập (Accessibility):** Đảm bảo các yếu tố UI dễ sử dụng cho mọi đối tượng người dùng.

## 2. Bảng màu (Color Palette)

Sử dụng các biến CSS của Tailwind/Shadcn UI để dễ dàng tùy chỉnh.

-   **Màu chính (Primary):** `hsl(222.2 47.4% 11.2%)` (Màu xanh đậm/đen than - `foreground`)
    -   Sử dụng cho các nút hành động chính, tiêu đề quan trọng.
-   **Màu nền (Background):** `hsl(210 40% 98%)` (Màu trắng ngà - `background`)
    -   Nền chung của ứng dụng.
-   **Màu phụ (Accent):** `hsl(217.2 91.2% 59.8%)` (Màu xanh dương - `primary`)
    -   Sử dụng cho các điểm nhấn, liên kết, trạng thái hover/active.
-   **Màu chữ (Text):**
    -   Chính: `hsl(222.2 47.4% 11.2%)` (Màu xanh đậm/đen than - `foreground`)
    -   Phụ: `hsl(215.4 16.3% 46.9%)` (Màu xám đậm - `muted-foreground`)
-   **Màu thành công (Success):** `hsl(142.1 76.2% 36.3%)` (Màu xanh lá cây)
-   **Màu lỗi (Error):** `hsl(0 84.2% 60.2%)` (Màu đỏ)
-   **Màu cảnh báo (Warning):** `hsl(48 96% 50%)` (Màu vàng)

## 3. Typography (Kiểu chữ)

-   **Font Family:** Sử dụng font sans-serif hệ thống hoặc một font Google Fonts phổ biến như Inter, Lato, hoặc Roboto để đảm bảo tính nhất quán và khả năng đọc.
    -   Ví dụ: `font-family: 'Inter', sans-serif;`
-   **Kích thước chữ:**
    -   **Tiêu đề lớn (H1):** `text-4xl` (ví dụ: 36px)
    -   **Tiêu đề (H2):** `text-3xl` (ví dụ: 30px)
    -   **Tiêu đề phụ (H3):** `text-2xl` (ví dụ: 24px)
    -   **Tiêu đề nhỏ (H4):** `text-xl` (ví dụ: 20px)
    -   **Văn bản chính (Body):** `text-base` (ví dụ: 16px)
    -   **Văn bản phụ/nhỏ:** `text-sm` (ví dụ: 14px), `text-xs` (ví dụ: 12px)
-   **Trọng lượng chữ (Font Weight):** Regular (400), Medium (500), Semi-bold (600), Bold (700).
-   **Chiều cao dòng (Line Height):** `line-height: 1.5;` cho văn bản chính để dễ đọc.

## 4. Khoảng cách và Bố cục (Spacing & Layout)

-   Sử dụng hệ thống khoảng cách dựa trên Tailwind CSS (ví dụ: `p-4`, `m-2`, `gap-x-4`).
-   **Grid System:** Sử dụng Flexbox và CSS Grid của Tailwind để tạo bố cục linh hoạt và phản hồi.
-   **Border Radius:** `rounded-md` (4-6px) cho các thành phần như nút, thẻ, input để tạo cảm giác mềm mại.
-   **Shadows:** `shadow-sm` hoặc `shadow-md` cho các thẻ (Card) để tạo chiều sâu nhẹ.

## 5. Iconography (Biểu tượng)

-   Sử dụng thư viện biểu tượng phổ biến và tương thích với Shadcn UI, ví dụ: [Lucide Icons](https://lucide.dev/).
-   Đảm bảo kích thước và màu sắc của biểu tượng nhất quán với ngữ cảnh sử dụng.

## 6. Các thành phần UI cụ thể (Component Styling)

### 6.1. Thanh điều hướng (Navbar)
-   **Nền:** `background` (trắng ngà).
-   **Chiều cao:** Cố định (ví dụ: 60px).
-   **Logo/Tên ứng dụng:** `text-xl` hoặc `text-2xl`, `font-bold`, màu `foreground`.
-   **Các liên kết/Nút:** `text-sm`, `font-medium`, màu `foreground` khi bình thường, màu `primary` khi hover/active.

### 6.2. Nút (Buttons)
-   **Primary Button:**
    -   Nền: `primary` (xanh dương).
    -   Chữ: `white`.
    -   Hover: Nền `primary` đậm hơn một chút.
    -   Border Radius: `rounded-md`.
-   **Secondary Button:**
    -   Nền: `muted` (xám nhạt).
    -   Chữ: `muted-foreground` (xám đậm).
    -   Hover: Nền `muted` đậm hơn.
-   **Outline Button:**
    -   Nền: `transparent`.
    -   Chữ: `foreground`.
    -   Border: `border border-input`.

### 6.3. Trường nhập liệu (Input Fields)
-   **Nền:** `background` (trắng ngà).
-   **Border:** `border border-input` (màu xám nhạt).
-   **Focus State:** `ring-2 ring-ring` (vòng sáng màu xanh dương nhẹ).
-   **Placeholder:** `muted-foreground`.
-   **Border Radius:** `rounded-md`.

### 6.4. Thẻ (Cards)
-   **Nền:** `background` (trắng ngà).
-   **Border:** `border border-border` (màu xám rất nhạt).
-   **Shadow:** `shadow-sm`.
-   **Border Radius:** `rounded-lg`.
-   **Nội dung:** Padding `p-4` hoặc `p-6`.

### 6.5. Bảng (Tables)
-   **Header:** Nền `muted` (xám nhạt), chữ `muted-foreground`, `font-semibold`.
-   **Rows:** Nền `background`, border dưới `border-b border-border`.
-   **Hover:** Nền `accent` nhẹ.

### 6.6. Thanh tiến độ (Progress Bar)
-   **Nền:** `secondary` (xám).
-   **Màu tiến độ:** `primary` (xanh dương).
-   **Border Radius:** `rounded-full`.

### 6.7. Thông báo (Alerts/Toasts)
-   **Success:** Nền xanh lá cây nhạt, chữ xanh lá cây đậm.
-   **Error:** Nền đỏ nhạt, chữ đỏ đậm.
-   **Info:** Nền xanh dương nhạt, chữ xanh dương đậm.
-   **Border Radius:** `rounded-md`.

## 7. Phản hồi tương tác (Interaction Feedback)

-   **Hover States:** Các nút, liên kết, thẻ sẽ có hiệu ứng hover nhẹ (thay đổi màu nền, màu chữ, hoặc shadow).
-   **Active States:** Hiệu ứng khi click (ví dụ: giảm độ sáng của nút).
-   **Loading Indicators:** Sử dụng spinner hoặc skeleton loaders cho các nội dung đang tải.

---

Tài liệu này cung cấp hướng dẫn chi tiết về các yếu tố UI để đảm bảo tính nhất quán và thẩm mỹ cho toàn bộ ứng dụng Quiz-Bank. Các giá trị cụ thể (như mã màu, kích thước font) sẽ được triển khai thông qua Tailwind CSS và các thành phần của Shadcn UI.