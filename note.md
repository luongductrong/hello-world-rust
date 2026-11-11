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

---

### Bài 2: Ownership, Borrowing & Slices ("Trái tim của Rust")

#### 1. Tại sao có Ownership? (So với C & JS/Java)
- **C:** tự `malloc`/`free` → nhanh nhưng dễ *double free* hoặc *use-after-free*.
- **JS/Java:** dùng Garbage Collector (GC) → an toàn nhưng tốn RAM/GC pause, nhiều biến cùng trỏ và sửa chung 1 object.
- **Rust:** **không dùng GC, không `free` tay** — compiler theo dõi chủ sở hữu (Owner) lúc compile và tự chèn `drop()` đúng lúc Owner ra khỏi scope `}`.
- **3 quy tắc vàng:** (1) Mỗi giá trị có đúng 1 Owner; (2) Cùng lúc chỉ có 1 Owner; (3) Owner hết scope → giá trị tự bị `drop()`.

#### 2. `Copy` (Stack) vs `Move` (Heap) & `Clone`
- **Kiểu `Copy`** (nằm gọn trên Stack: `i32`, `f64`, `bool`, `char`, tuple toàn kiểu `Copy`): `let y = x;` sẽ sao chép giá trị, cả `x` và `y` đều dùng bình thường.
- **Kiểu sở hữu Heap** (như `String`, `Vec<T>`):
  - `let s2 = s1;` **KHÔNG** tạo 2 tham chiếu trỏ chung như JS/Java, mà là **Move** (chuyển quyền sở hữu sang `s2`, vô hiệu hóa `s1` ngay lúc compile để tránh *double free*).
  - Truyền `String` (không phải `&`) vào hàm cũng làm **Move** quyền sở hữu vào hàm đó; kết thúc hàm biến bị `drop()`.
  - Muốn nhân bản thật trên Heap (deep copy) phải gọi tường minh `.clone()`: `let s2 = s1.clone();`.

#### 3. References & Borrowing (`&T` vs `&mut T`)
- **Reference (`&`)** là tham chiếu an toàn (không bao giờ `null`, không bao giờ trỏ vào vùng đã `drop`), giúp hàm **mượn** dữ liệu mà không lấy mất Ownership.
- **Quy tắc Borrow Checker (chống Data Race lúc compile):**
  - **`&T` (Shared / Immutable Reference):** Mượn để đọc — **nhiều nơi** có thể cùng giữ `&T` một lúc, không được sửa.
  - **`&mut T` (Exclusive / Mutable Reference):** Mượn để sửa — tại 1 thời điểm chỉ được tồn tại **duy nhất 1 `&mut T`**, và không ai khác được giữ cả `&T` lẫn `&mut T` cùng lúc.
- **Non-Lexical Lifetimes (NLL):** Một lần mượn kết thúc ngay tại **dòng cuối cùng mà biến tham chiếu đó còn được sử dụng** (chứ không cần đợi tới dấu `}` cuối block).
- Khi sửa giá trị nguyên thủy qua `&mut`, dùng toán tử giải tham chiếu `*` (ví dụ `*n += 1;`); khi gọi method (như `s.push_str("!")`), Rust tự động dereference.

#### 4. Slices (`&str` và `&[T]`)
- **Slice** là tham chiếu tới một **phần liên tiếp** của dữ liệu (chỉ gồm con trỏ + độ dài `len`, không sở hữu Heap).
- **`&str` vs `&String`:**
  - `String`: sở hữu chuỗi trên Heap, co giãn được (`push`, `push_str`, `clear`).
  - `&str`: lát cắt chuỗi chỉ đọc (*string slice*). Luôn ưu tiên nhận `s: &str` cho tham số hàm chỉ đọc vì nó nhận được cả `&String` (nhờ *Deref coercion*), string literal `"abc"`, lẫn đoạn cắt `&s[0..3]`.
  - Chỉ dùng `&mut String` khi hàm cần thay đổi độ dài/nội dung chuỗi gốc, và trả về `String` khi hàm tạo chuỗi mới.
- **Tương tự với mảng/Vec:** dùng `&[T]` (ví dụ `nums: &[i32]`) thay vì `&Vec<i32>` để nhận được cả array `[i32; N]`, `Vec<i32>`, lẫn lát cắt `&arr[1..3]`.
- **Lưu ý UTF-8:** Cắt chuỗi `&s[start..end]` là cắt theo **byte**, nếu cắt giữa chừng một ký tự tiếng Việt/emoji sẽ bị panic! Nên dùng `char_indices()` hoặc `.find(...)` để lấy vị trí byte an toàn.
- **Expression cuối hàm vs `return` sớm:** Chỉ dùng expression (không `return`, không `;`) ở dòng cuối cùng trả về kết quả của khối/hàm; khi thoát sớm giữa vòng `for` / `loop` (early return), **bắt buộc dùng `return ...;`**.
- **Hiểu thông báo lỗi `clear()` + slice:** `word` mượn `&sentence` kéo dài tới dòng `println!("{word}")` cuối cùng; nên khi `sentence.clear()` xen vào giữa đòi `&mut sentence`, Borrow Checker báo lỗi **ngay tại dòng `clear()`** vì chính thao tác đòi quyền sửa đó xung đột với lượt mượn đọc đang còn sống.

