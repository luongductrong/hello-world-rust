### Bài 6: Chiều sâu Hệ thống Kiểu — Lifetimes & Advanced Traits

#### 1. Bản chất của Lifetime (`'a`) & Cơ chế mượn liên kết

- **Bản chất Lifetime:** Mọi tham chiếu (`&T` hoặc `&mut T`) trong Rust đều gắn liền với một phạm vi hiệu lực (scope) trong bộ nhớ.
- **Quy tắc Tỉnh lược (Lifetime Elision):** Hầu hết các hàm 1 tham số vào, 1 tham số ra Rust đều tự suy luận được thời gian sống nên ta không cần gõ `'a`.
- **Tại sao phải khai báo tường minh `'a`?**
  - Khi một hàm nhận **từ 2 tham chiếu trở lên** và **trả về 1 tham chiếu**, compiler phân tích hàm độc lập (_Modular Analysis_) nên không thể đoán giá trị trả về mượn từ tham số nào:

    ```rust
    // LỖI COMPILE: Không biết &str trả về mượn từ x hay y
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() { x } else { y }
    }

    // HỢP LỆ: Khai báo generic lifetime 'a
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    ```

- **Ý nghĩa cốt lõi của `'a`:** Ký hiệu `'a` **KHÔNG LÀM THAY ĐỔI** thời gian sống của biến! Nó chỉ là **hợp đồng (contract)** cam kết với Borrow Checker: _Tham chiếu trả về sẽ sống tối đa bằng thời gian sống của tham số ngắn ngủi nhất trong các tham số gắn `'a'`_.
- **So sánh C vs Java/JS vs Rust:**
  - **C:** Trỏ con trỏ `char*` tự do $\to$ nguy cơ cực cao dính lỗi bảo mật tử huyệt _Dangling Pointer / Use-after-free_ khi trỏ vào vùng nhớ đã bị giải phóng.
  - **Java / JS:** Mọi object nằm trên Heap được GC dọn dẹp $\to$ an toàn nhưng tốn RAM và đánh đổi hiệu năng runtime (_GC pause_).
  - **Rust:** Tốc độ tối đa như C (Zero-cost abstraction, trỏ thẳng không cần cấp phát mới), nhưng an toàn tuyệt đối 100% như Java/JS vì Borrow Checker chặn đứng mọi tham chiếu lơ lửng ngay lúc compile.

#### 2. Struct chứa tham chiếu (Zero-Allocation / Zero-Copy View)

- **Định nghĩa:** Thay vì struct luôn phải tự sở hữu dữ liệu (`String`, `Vec<u8>`), ta tạo struct chỉ mượn lát cắt dữ liệu từ nơi khác:

  ```rust
  struct RequestView<'a> {
      method: &'a str,
      path: &'a str,
      token: Option<&'a str>,
  }

  impl<'a> RequestView<'a> {
      // Khai báo <'a> ngay sau impl
  }
  ```

- **Quy tắc sắt đá:** Một struct chứa tham chiếu `'a` **không thể sống lâu hơn** vùng nhớ mà nó đang mượn. Nếu biến sở hữu gốc bị drop thì struct đó lập tức bị compiler vô hiệu hóa.
- **Ứng dụng thực tế:** High-Performance Parsers & Network Servers. Đọc một buffer dữ liệu 100MB và tạo hàng ngàn struct `View` xử lý mà **không tốn 1 byte RAM cấp phát mới (Heap allocation)**.

#### 3. Anonymous Lifetime (`'_`) & Ứng dụng (Tauri State)

- **Ý nghĩa:** Ký hiệu `'_` báo cho compiler: _"Kiểu dữ liệu này có chứa tham chiếu, hãy tự suy luận lifetime của nó theo ngữ cảnh, tôi không cần đặt tên biến `'a'`"_.
- **Cú pháp:** Dùng khi nhận struct có lifetime làm tham số hàm:
  ```rust
  fn inspect_view(view: &RequestView<'_>) {
      println!("Method: {}", view.method);
  }
  ```
- **Ứng dụng trong Tauri:**
  ```rust
  #[tauri::command]
  fn greet(state: tauri::State<'_, AppState>) -> String {
      format!("App: {}", state.app_name)
  }
  ```
  `tauri::State<'_, AppState>` giữ tham chiếu mượn AppState do Tauri quản lý. Dùng `'_` giúp chữ ký hàm gọn gàng, không cần khai báo generic lifetime `<'a>` rối mắt.

#### 4. Toolbox: Cắt chuỗi không cấp phát Heap (Zero-Copy Slicing)

- **`s.split_whitespace()`:** Iterator tách chuỗi theo khoảng trắng thành các lát cắt `&str`.
- **`s.split_once(pat)`:** Tách chuỗi tại ký tự phân cách đầu tiên $\to$ trả về `Option<(&str, &str)>` (cực xịn cho parsing `key:value`).
- **`s.strip_prefix(prefix)`:** Cắt bỏ tiền tố nếu khớp, trả về `Option<&str>` phần đuôi còn lại mà không tốn thêm byte RAM nào:
  ```rust
  let payload = "payload:width=800".strip_prefix("payload:"); // Some("width=800")
  ```
- **`s.lines()`:** Iterator duyệt qua từng dòng `&str` trong chuỗi văn bản lớn.

#### 5. Các kinh nghiệm thực chiến đúc kết từ Buổi 6.1

- **Quy tắc Tỉnh lược Lifetime (Lifetime Elision Rules):**
  Nếu hàm chỉ có **1 tham số đầu vào là tham chiếu**, compiler sẽ tự động gán lifetime của tham chiếu đó cho tất cả các tham chiếu ở giá trị trả về:
  ```rust
  // Viết gọn:
  fn parse_command(raw: &str) -> Result<CommandView<'_>, &str>
  // Compiler tự động hiểu thành:
  fn parse_command<'a>(raw: &'a str) -> Result<CommandView<'a>, &'a str>
  ```
- **Phân biệt vị trí đặt Anonymous Lifetime `'_`:**
  - `&CommandView<'_>`: Tham chiếu mượn một `CommandView`, trong đó lifetime bên trong struct (`'a`) được để compiler tự suy luận (cách viết chuẩn).
  - `&'_ CommandView`: Ký hiệu `'_` gắn vào bản thân con trỏ tham chiếu `&` (thường thừa vì `&` mặc định luôn được suy luận).
- **Xử lý `write!` trong `Display` trait:**
  Macro `write!(f, ...)` trả về `Result<(), std::fmt::Error>`. Cách viết chuẩn mực trong Rust không phải dùng `let _ = write!(...)` mà là dùng toán tử `?`:
  ```rust
  write!(f, "Action: {}; Target: {}", self.action, self.target)?;
  if let Some(payload) = self.payload {
      write!(f, "; Payload: {}", payload)?;
  }
  Ok(())
  ```
- **So sánh 2 Option không cần `unwrap()` (Pattern Matching tuple):**
  Thay vì kiểm tra `.is_some()` rồi gọi `.unwrap()`, gom 2 Option thành tuple và match:
  ```rust
  match (cm1.payload, cm2.payload) {
      (Some(p1), Some(p2)) => if p1.len() >= p2.len() { Some(p1) } else { Some(p2) },
      (Some(p1), None) => Some(p1),
      (None, Some(p2)) => Some(p2),
      (None, None) => None,
  }
  ```
- **`.map()` vs `.and_then()` trên `Option` / `Result`:**
  - `.map(f)`: Dùng khi closure `f` trả về một **giá trị thông thường** `T` $\to$ kết quả tự động bọc lại thành `Option<T>`.
  - `.and_then(f)`: Dùng khi closure `f` tự nó đã trả về một **`Option<U>`** (hoặc `Result`) $\to$ `.and_then()` làm phẳng (_flatten/monadic bind_), tránh tạo ra kiểu lồng nhau `Option<Option<U>>`.
- **Thí nghiệm "Dangling Pointer" bị Compiler chặn đứng:**

  ```rust
  let view;
  {
      let temp = String::from("QUERY temp_cmd");
      view = parse_command(&temp).unwrap();
  } // <-- `temp` bị DROP và thu hồi bộ nhớ tại đây!

  println!("{:?}", view); //  COMPILE ERROR: `temp` does not live long enough!
  ```

  Biến `view` được khai báo ngoài scope, nhưng dữ liệu mà nó mượn (`temp`) lại chết bên trong scope con. Rust phát hiện ra ngay lúc compile và ngăn chặn hoàn toàn lỗi _Use-after-free_!

#### 6. Phân biệt `'static` Lifetime (`&'static str`) vs Trait Bound `T: 'static`

- **Bản chất 2 ngữ cảnh của từ khóa `'static`:**
  - **Lifetime của tham chiếu (`&'static T`):** Vùng nhớ mà tham chiếu trỏ tới tồn tại vĩnh viễn trong suốt thời gian chạy của app (thường nằm ở phân vùng tĩnh của file binary `.exe` như `"HELLO"`).
  - **Trait Bound trên kiểu (`T: 'static`):** **KHÔNG** bắt buộc `T` phải sống mãi mãi! Nó chỉ khẳng định: Kiểu `T` là kiểu **tự sở hữu toàn bộ dữ liệu** (Owned type) hoặc chỉ chứa tham chiếu `'static`, **hoàn toàn không mượn bất kỳ tham chiếu ngắn hạn nào**.
- **So sánh các kiểu dữ liệu với `T: 'static`:**
  - `i32`, `bool`, `f64` $\to$ **Thoả mãn** (tự lưu trên stack).
  - `String`, `Vec<u8>`, struct tự sở hữu $\to$ **Thoả mãn** (tự quản lý bộ nhớ heap riêng, khi chuyển giao quyền sở hữu không phụ thuộc vào scope nào).
  - `&'static str` $\to$ **Thoả mãn** (tham chiếu sống vĩnh cửu).
  - `&'a str`, `&'a i32`, struct chứa `&'a ...` $\to$ **KHÔNG thoả mãn** (mượn từ nơi khác, nguy cơ bị dangling pointer khi luồng cha bị giải phóng).
- **Ý nghĩa với Đa luồng & Async:**
  Hàm `std::thread::spawn` hoặc `tokio::spawn` ép kiểu gửi sang thread mới phải thoả mãn `T: 'static + Send`. Luồng con có thể sống lâu hơn luồng cha, nếu cho phép truyền tham chiếu mượn ngắn hạn từ luồng cha sang, luồng cha chết sẽ làm luồng con trỏ vào vùng nhớ rác gây crash.

#### 7. Khi nào cần dùng `Box::leak` (Ứng dụng thực tế)

- **Tại sao cần `Box::leak` khi `String` vốn dĩ đã thoả mãn `T: 'static`?**
  - **Trường hợp 1 (Third-party Crate / Framework):** Rất nhiều thư viện (như Logger, Metrics Prometheus, OpenTelemetry) thiết kế hàm nhận cứng `&'static str` (thay vì nhận generic `T` hay `String`) để tối ưu zero-allocation. Khi dữ liệu chuỗi được đọc từ file config lúc runtime (`String`), cách duy nhất để chuyển vào là dùng `Box::leak(s.into_boxed_str())`.
  - **Trường hợp 2 (Global Read-Only Config):** Đọc file cấu hình 50MB lúc app khởi động, muốn hàng trăm thread đọc song song với tốc độ con trỏ thô cao nhất mà không cần tranh chấp khóa lock (`Mutex`/`RwLock`) hay bọc `Arc`. Dùng `Box::leak` biến nó thành tham chiếu `'static` toàn cục.
  - **Trường hợp 3 (FFI / Foreign Function Interface):** Chuyển con trỏ chuỗi từ Rust sang C engine / WebAssembly / iOS native code mà không bị Rust tự động drop khi hàm Rust kết thúc.

#### 8. Các kinh nghiệm thực chiến đúc kết từ Buổi 6.2

- **Chữ ký hàm generic nhận `T: 'static`:**
  ```rust
  fn emit_event<T>(event: T)
  where
      T: std::fmt::Debug + Display + 'static,
  {
      let type_name = std::any::type_name::<T>();
      println!("Type name: {type_name} - Value: {event}");
  }
  ```
- **Thao tác leak chuỗi:**
  `Box::leak(string.into_boxed_str()) -> &'static str`: Cố ý "quên" việc thu hồi bộ nhớ heap của chuỗi đó, đổi lại ta nhận được một tham chiếu tĩnh sống vĩnh viễn suốt phiên chạy của ứng dụng.
- **`slice` mượn từ biến cục bộ bị compiler chặn ngay:**
  ```rust
  let local = String::from("local");
  let slice: &str = &local;
  emit_event(slice); //  LỖI: `local` does not live long enough
  ```
  `local` chỉ sống trong hàm hiện tại, tham chiếu `slice` mang lifetime cục bộ nên không thoả mãn ràng buộc `'static`.
