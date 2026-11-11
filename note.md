### Bài 0: Làm chủ Cargo & quy trình code Rust hàng ngày

1. **`cargo check`** — _Kiểm tra code nhanh nhất (dùng liên tục lúc đang viết)_
2. **`cargo run`** (và `cargo build`) — _Biên dịch và chạy_
   - `cargo build`: dịch code thành file thực thi `target\debug\hello-world.exe` (chế độ dev, dịch nhanh để debug). Khi cần bản tối ưu tốc độ để phát hành thì dùng `cargo build --release` (nằm trong `target\release\`).
   - `cargo run`: tự động build nếu code có sửa rồi chạy luôn file `.exe` đó.

3. **`cargo fmt`** — _Tự động làm đẹp code theo chuẩn Rust_
   - `cargo fmt`: tự căn thụt đầu dòng, khoảng trắng, sắp xếp `use` trực tiếp vào file.
   - `cargo fmt --check`: chỉ kiểm tra xem file có đúng chuẩn format chưa (không sửa file).

4. **`cargo clippy`** — _Trợ lý bắt "code mùi" & gợi ý viết chuẩn Rust_

5. **`cargo test`** & **`cargo add`** _(sẽ dùng ở các chặng sau)_
   - `cargo test`: chạy các bài test tự động trong project.
   - `cargo add <tên_crate>`: cài thêm thư viện ngoài vào [`Cargo.toml`](./Cargo.toml) (không cần mở file gõ tay).

**Nhịp làm việc chuẩn khi mở VS Code:**
Gõ code (nhìn gợi ý/báo đỏ từ `rust-analyzer`) → Lưu (`Ctrl + S` tự `fmt`) → `cargo clippy` / `cargo check` → `cargo run` để kiểm chứng kết quả.

---

### Bài 1: Biến, Kiểu dữ liệu & Hàm (Đối chiếu TS / Java / C)

#### 1. Biến: `let`, `let mut`, `const` và Shadowing

- **`let` mặc định bất biến (immutable):** giống `const` trong JS/TS hoặc `final` biến cục bộ trong Java. Muốn sửa giá trị phải khai báo **`let mut`**.
- **`const` trong Rust:** hằng số _compile-time_ (giống `static final` hằng số trong Java), **bắt buộc** ghi rõ kiểu và viết HOA (`const MAX_RETRY: u32 = 3;`).
- **Shadowing:** khai báo lại `let x = ...` cùng tên trong cùng scope là hợp lệ, thậm chí **đổi được kiểu dữ liệu** (ví dụ `let input = "42"; let input: i32 = 42;`), khác với JS/TS/Java báo lỗi trùng tên biến.

#### 2. Kiểu dữ liệu cơ bản (Scalar & Compound)

- **Số nguyên:** `i8..i128` (có dấu), `u8..u128` (không dấu). Mặc định là `i32`. Riêng `usize` / `isize` theo kích thước con trỏ hệ thống (32/64-bit, dùng làm index mảng như `size_t` trong C).
- **Số thực:** `f64` (mặc định, như `number` JS / `double` Java) và `f32`.
- **Khác:** `bool` (`true`/`false`), `char` (4 bytes Unicode, dùng dấu nháy đơn `'🦀'`), chuỗi dùng nháy kép.
- **Tuple:** `(i32, f64, bool)` — giống tuple trong TS, truy cập `tup.0` hoặc destructuring `let (a, b, c) = tup;`.
- **Array:** `[i32; 3]` — độ dài **cố định lúc compile** (như mảng tĩnh trong C). Mảng co giãn giống `Array` JS / `ArrayList` Java là `Vec<T>` (học sau).

#### 3. Hàm & Tư duy Expression

- Tham số và giá trị trả về **bắt buộc** ghi rõ kiểu: `fn add(a: i32, b: i32) -> i32`.
- **Statement vs Expression:**
  - Dòng có `;` là _statement_ (thực hiện hành động, không trả về giá trị).
  - Dòng cuối trong `{ ... }` **không có `;`** là _expression_ — tự động là giá trị trả về (giống arrow function `=> a + b` trong JS). Nếu thêm `;` vào cuối, hàm trả về `()` (_unit type_, tương đương `void`).
- **Block `{ ... }`:** Là một _expression_ tạo scope cục bộ và trả về dòng cuối không có `;` (không phải anonymous function/closure; closure thật trong Rust dùng `|x| x + 1`, tương đương `(x) => x + 1` ở JS).
- **Lưu ý số thực & format:** `f64` dùng chuẩn IEEE 754 nên có sai số nhị phân giống JS (ví dụ `37.0 * 1.8 + 32.0 = 98.60000000000001`). Khi in dùng `println!("{fahrenheit:.1}");` để hiển thị 1 chữ số thập phân.

#### 4. Điều khiển luồng (`if/else`, `loop`, `while`, `for`)

- **Không có toán tử 3 ngôi `? :`** — dùng thẳng `if/else` như một expression: `let status = if temp > 37.5 { "Sốt" } else { "Bình thường" };`.
- Điều kiện sau `if` / `while` **bắt buộc phải là `bool`** (không có truthy/falsy như JS hay `0`/`1` như C), và **không cần ngoặc tròn** `(...)`.
- **3 kiểu vòng lặp:**
  - `loop { ... }`: lặp vô tận (thay `while (true)`), có thể `break value;` để trả về giá trị.
  - `while cond { ... }`: lặp theo điều kiện.
  - `for i in 1..=5 { ... }`: lặp theo range (`1..5` là 1→4, `1..=5` là 1→5) hoặc duyệt phần tử. Rust **không có** `for (i = 0; i < n; i++)` kiểu C.

- Tư duy "mọi thứ đều là expression"

  `let label = if x > 0 { "dương" } else { "không dương" };`

#### 5. Tách file thành module cơ bản (`mod` & `pub`)

- Trong [`src/main.rs`](./src/main.rs), khai báo `mod lesson_1;` để báo compiler nạp file [`src/lesson_1.rs`](./src/lesson_1.rs) (khác JS/TS là `import` tự kéo file, trong Rust phải có `mod` mới nạp file).
- Hàm trong module mặc định là **private**; muốn file ngoài gọi được phải thêm `pub fn ...`. Gọi qua `lesson_1::run();` hoặc `use lesson_1::run;` rồi `run();`.

#### 6. Đọc input bàn phím (`std::io`) & chuyển chuỗi sang số (`parse`)

- Đọc 1 dòng từ terminal: tạo `let mut input = String::new();` rồi gọi `std::io::stdin().read_line(&mut input)` (`read_line` nối thêm cả `\r\n` vào cuối nên cần `.trim()`).
- Dùng `print!("...")` không tự xuống dòng cần gọi `io::stdout().flush().unwrap();` để đẩy chữ ra màn hình trước khi dừng chờ nhập.
- Ép kiểu chuỗi → số an toàn với `parse()` (trả về `Result` thay vì `NaN` như JS):
  ```rust
  let guess: u32 = match input.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
  };
  ```
