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

#### 9. Associated Types (`type Item;`) vs Generic Trait (`Trait<T>`)

- **Bản chất khác biệt:**
  - **Generic Trait (`trait Trait<T>`):** Quan hệ 1-Nhiều. Một kiểu cụ thể có thể `impl Trait<A>`, `impl Trait<B>` nhiều lần (ví dụ `From<T>`).
  - **Associated Type (`type Item;`):** Quan hệ 1-1. Một kiểu cụ thể khi cài đặt Trait chỉ gắn liền với **duy nhất 1 kiểu đầu ra** (ví dụ `Iterator`, `Worker`). Tránh tình trạng người dùng gọi hàm mơ hồ hoặc không mong muốn.
- **Lợi ích chữ ký hàm:** Giúp code ngắn gọn gấp đôi:
  - Generic: `fn run<T, W: Worker<T>>(w: W) -> T` (phải khai báo cả `T` lẫn `W`).
  - Associated Type: `fn run<W: Worker>(w: W) -> W::Response` (chỉ cần khai báo generic `W`, kiểu kết quả tự đi kèm thông qua `W::Response`).

#### 10. Trait Object Bounds Đa luồng (`dyn Trait + Send + Sync`)

- **Vấn đề ở môi trường đa luồng:** Trong đơn luồng, `Box<dyn Trait>` là đủ. Nhưng khi chuyển giao sang thread khác (`std::thread::spawn` hoặc `tokio::spawn`), compiler sẽ từ chối nếu Trait Object chưa được chứng nhận an toàn luồng.
- **Cú pháp Trait Object an toàn đa luồng:**
  ```rust
  Box<dyn ThreadSafeJob + Send + Sync>
  ```
- **Cơ chế Ép kiểu Trait Object (Unsize Coercion):**
  - Khi truyền `Box<PrintJob>` vào hàm nhận `Box<dyn ThreadSafeJob + Send + Sync>`, Rust **tự động ép kiểu (coerce)** vì kiểu đích của hàm đã được xác định tường minh lúc compile.
  - Khác với `vec![Box::new(a), Box::new(b)]` trước đây: marco `vec![]` suy luận theo phần tử đầu tiên nên mới cần khai báo kiểu rõ ràng `Vec<Box<dyn Trait>>`.

#### 11. Các kinh nghiệm thực chiến đúc kết từ Buổi 6.3

- **Khai báo Trait có Associated Type:**
  ```rust
  trait Worker {
      type Response;
      fn perform(&self) -> Self::Response;
      fn name(&self) -> &str;
  }
  ```

---

### Bài 7: Concurrency & Shared State — Đa luồng an toàn (Fearless Concurrency)

#### 1. OS Thread (`std::thread::spawn`), `move ||` & Cơ chế hoạt động

- **OS Thread thực thụ:** `std::thread::spawn` tạo một Kernel Thread do Hệ điều hành quản lý, thực thi song song trên các core CPU vật lý.
- **`JoinHandle<T>`:**
  - Kết quả trả về của `spawn`: `let handle = std::thread::spawn(...)`.
  - Gọi `handle.join().unwrap()` để tạm dừng thread hiện tại (blocking wait) chờ thread con hoàn thành và nhận kết quả trả về `T`.
  - Nếu không gọi `.join()`, khi main thread kết thúc, tiến trình (process) sẽ thoát ngay lập tức và khai tử toàn bộ thread con dù chưa chạy xong.
- **Bắt buộc dùng `move ||`:**
  - Vì thread mới có thể sống lâu hơn thread cha sinh ra nó, hàm `std::thread::spawn` đòi hỏi closure phải thoả mãn trait bound `'static`.
  - Từ khóa `move` ép closure chuyển giao toàn bộ quyền sở hữu (ownership) của các biến vào trong thread, không mượn tham chiếu ngắn hạn từ thread cha.

#### 2. Marker Traits: `Send` và `Sync` (Bí mật phòng chống Data Race lúc compile)

- **`T: Send` (Chuyển giao quyền sở hữu an toàn):**
  - Cho phép chuyển `T` từ thread này sang thread khác.
  - Hầu hết các kiểu trong Rust đều là `Send` (`i32`, `String`, `Vec<T>`).
  - **Kiểu KHÔNG `Send`:** `Rc<T>` (vì bộ đếm tham chiếu dùng phép toán số học thông thường `count += 1`, nếu 2 thread cùng tăng sẽ gây Data Race làm hỏng bộ nhớ Heap). Compiler lập tức chặn: `the trait Send is not implemented for Rc<String>`.
- **`T: Sync` (Chia sẻ tham chiếu đồng thời an toàn):**
  - Cho phép nhiều thread cùng mượn đọc `&T` đồng thời.
  - Định lý Rust: `T: Sync` $\iff$ `&T: Send`.
  - `RefCell<T>` KHÔNG phải là `Sync` vì cơ chế mượn động lúc runtime không an toàn đa luồng. `Mutex<T>` và `RwLock<T>` là `Sync`.
- **Auto Traits:** Compiler tự động suy luận `Send` và `Sync` dựa trên các trường bên trong struct, không cần lập trình viên tự tay implement.

#### 3. Phân biệt Thread vs Process & Tối ưu bộ nhớ

- **Thread vs Process:**
  - Nhiều threads chạy chung trong **cùng 1 tiến trình (process)**, chia sẻ cùng không gian địa chỉ ảo (Virtual Address Space), Heap và tài nguyên hệ thống.
  - Tạo thread mới KHÔNG sinh ra process mới.

#### 4. Toolbox: Khởi tạo nhanh Vec & Cắt Chunks

- **Tạo nhanh `Vec` từ Range:**
  ```rust
  let data: Vec<u64> = (1..=100_000).collect();
  // hoặc dùng cú pháp turbofish / type inference:
  let data = (1_u64..=100_000).collect::<Vec<_>>();
  ```
- **Chia mảng thành các phần bằng nhau với `.chunks(size)`:**

  ```rust
  let chunks: Vec<&[u64]> = data.chunks(25_000).collect();
  ```

  - `data.chunks(n)` trả về một iterator các slice mượn `&[T]`, mỗi slice có tối đa `n` phần tử.
  - Để chuyển quyền sở hữu cho từng thread: `chunk.to_vec()` (nhân bản lát cắt thành một `Vec<T>` độc lập trên heap).

#### 5. Các kinh nghiệm thực chiến đúc kết từ Buổi 7.1

- **Thu thập Handles & Gom kết quả:**

  ```rust
  let mut handles = Vec::new();
  for (id, chunk) in chunks.iter().enumerate() {
      let own_chunk = chunk.to_vec();
      let handle = std::thread::spawn(move || process_chunk(id + 1, own_chunk));
      handles.push(handle);
  }

  let total: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
  ```

- **Thứ tự hoàn thành bất định (Non-deterministic completion):**
  Các worker in log hoàn thành không theo thứ tự 1, 2, 3, 4 vì Hệ điều hành (OS Scheduler) điều phối các thread chạy song song độc lập trên các CPU core khác nhau.

#### 6. Interior Mutability: Phá vỡ giới hạn tham chiếu bất biến (`&self`)

- **Vấn đề thực tế:** Khi chia sẻ App State hay Cache, các hàm chỉ nhận tham chiếu mượn đọc bất biến `&self`. Nếu theo luật thường của Rust, ta không thể sửa đổi trạng thái bên trong.
- **Bản chất Interior Mutability:** Vỏ bọc bên ngoài là bất biến (`&self`), nhưng kiểu dữ liệu bên trong (`RefCell`, `Mutex`, `RwLock`) cung cấp cơ chế an toàn để sửa đổi dữ liệu (mutate) lúc runtime.

#### 7. `RefCell` (Đơn luồng) vs `Mutex` vs `RwLock` (Đa luồng)

| Tiêu chí            | `RefCell<T>`                         | `Mutex<T>`                          | `RwLock<T>`                               |
| :------------------ | :----------------------------------- | :---------------------------------- | :---------------------------------------- |
| **Môi trường**      | **Đơn luồng** (Single-thread)        | **Đa luồng** (Multi-thread)         | **Đa luồng** (Multi-thread)               |
| **Marker Traits**   | Không `Sync` (cấm truyền qua thread) | `Send + Sync` (nếu `T: Send`)       | `Send + Sync` (nếu `T: Send + Sync`)      |
| **Khi xung đột**    | **Panic Crash runtime** ngay lập tức | **Block / Ngủ chờ** đến khi mở khóa | **Block / Ngủ chờ** khi có luồng ghi      |
| **Cơ chế truy cập** | 1 ghi HOẶC nhiều đọc                 | 1 luồng độc quyền duy nhất          | **Nhiều luồng cùng đọc**, 1 ghi độc quyền |
| **Tối ưu cho**      | Thao tác đơn luồng nhẹ nhàng         | Tác vụ ghi thường xuyên             | Tác vụ **Read-Heavy** (đọc áp đảo ghi)    |

#### 8. Cơ chế Tự động Mở khóa RAII (Drop Guard) & Cạm bẫy Self-Deadlock

- **RAII Auto-Unlock:** Khi gọi `.lock().unwrap()` hay `.write().unwrap()`, kết quả trả về là một `Guard` thông minh (`MutexGuard`, `RwLockWriteGuard`). Khi biến guard ra khỏi scope `{}` (hoặc gọi `drop(guard)`), Rust **tự động mở khóa 100%**, không bao giờ lo quên unlock như Java/C++.
- **Cạm bẫy Tự khóa chết (Self-Deadlock):**
  - Nếu trên cùng 1 thread, ta đang giữ `read()` guard mà lại gọi `write()` $\to$ Luồng ghi ngồi chờ luồng đọc kết thúc, nhưng luồng đọc lại không thể kết thúc vì thread đang bị block chờ luồng ghi $\to$ **Deadlock vĩnh viễn**!
  - **Giải pháp:** Chủ động bọc scope `{ let _t = ...; }` hoặc gọi `drop(_t)` để nhả khóa đọc trước khi xin khóa ghi.

#### 9. Các kinh nghiệm thực chiến đúc kết từ Buổi 7.2

- **Kiểu trả về của `.borrow()`:** Trả về con trỏ thông minh `Ref<T>` (triển khai `Deref<Target = T>`) chứ không phải `T`, cho phép duyệt mượn trực tiếp mà không cần clone dữ liệu:
  ```rust
  for (i, log) in self.logs.borrow().iter().enumerate() { ... }
  ```
- **Kích nổ Panic của `RefCell` khi vi phạm Borrow Rules lúc Runtime:**
  ```rust
  let _r = logger.logs.borrow(); // Đang mượn đọc
  logger.logs.borrow_mut().push(...); // PANIC CRASH: already borrowed: BorrowMutError
  ```
  Khác với compiler chặn lúc gõ code, lỗi này xảy ra khi chương trình đang chạy.
- **Mẫu hình cập nhật an toàn với `Mutex` và `RwLock`:**
  - `Mutex`: `*self.request_count.lock().unwrap() += 1;`
  - `RwLock`: `*self.theme.write().unwrap() = new_theme.to_string();`

#### 10. Pattern chuẩn mực `Arc<Mutex<T>>` & `Arc<RwLock<T>>` (Mô hình App State)

- **Tại sao phải ghép đôi `Arc` với `Mutex`/`RwLock`?**
  - Một mình `Mutex<T>`: Bảo vệ dữ liệu không bị data race, nhưng **không thể chia sẻ quyền sở hữu** sang nhiều thread (luật move ownership chỉ cho đi 1 lần).
  - Một mình `Arc<T>`: Cho phép **đa sở hữu (multi-ownership)** giữa nhiều thread, nhưng chỉ cấp quyền **mượn đọc bất biến (`&T`)**, không ai được sửa dữ liệu.
  - **Ghép đôi `Arc<Mutex<T>>` / `Arc<RwLock<T>>`:** Tạo nên mô hình quản trị App State hoàn hảo. `Arc` cấp quyền truy cập vùng nhớ Heap cho nhiều luồng, `Mutex`/`RwLock` kiểm soát việc đọc/ghi an toàn.
- **Ứng dụng thực tế:** Đây là kiến trúc chuẩn mực của `tauri::State` trong Tauri và `axum::extract::State` trong backend Axum.

#### 11. Bộ đếm tham chiếu `Arc::clone` vs `Arc::strong_count`

- **`Arc::clone(&state)`:**
  - Không hề nhân bản dữ liệu thật trên Heap.
  - Chỉ thực hiện phép toán nguyên tử cấp CPU (_Atomic increment_) tăng bộ đếm lên `+1` và tạo ra một con trỏ thông minh mới trỏ tới cùng vùng nhớ. Chi phí cực kỳ nhẹ (chỉ vài nano-giây).
  - Khi một thread hoàn thành, biến clone trong thread đó bị drop $\to$ bộ đếm tự động `-1`.
  - Khi bộ đếm về `0` $\to$ bộ nhớ Heap tự động được giải phóng hoàn toàn.
- **`Arc::strong_count(&state)`:**
  - Trả về số lượng tham chiếu đang cùng sở hữu vùng nhớ này.
  - Giúp giám sát vòng đời bộ nhớ: kiểm tra ban đầu là `1`, tăng lên theo số thread đang chạy, và trở về đúng `1` khi toàn bộ các thread con đã `.join()`.

#### 12. Các kinh nghiệm thực chiến đúc kết từ Buổi 7.3

- **Quy tắc vàng "Clone trước khi spawn":**
  Luôn gọi `let state_clone = Arc::clone(&shared_state);` ngay trước khi gọi `std::thread::spawn(move || ...)`.
- **An toàn tuyệt đối trong điều kiện tranh chấp cao (High-concurrency):**
  Dù các giao dịch nạp và rút tiền từ 10 OS threads chạy xen kẽ bất định do Hệ điều hành lập lịch, dữ liệu số dư và danh sách lịch sử giao dịch không hề bị sai lệch hay mất mát dù chỉ 1 đồng.
- **Cơ chế Poisoned Mutex (`PoisonError`):**
  Lý do ta luôn gọi `.lock().unwrap()`: Nếu một thread trước đó đang cầm lock mà bị `panic!`, Mutex sẽ bị đánh dấu là "nhiễm độc". Các thread sau gọi `.lock()` sẽ nhận về `Err(PoisonError)` để tránh tiếp tục thao tác trên dữ liệu đã bị lỗi dở dang.

---

### Bài 8: Chuẩn hóa Xử lý Lỗi Công nghiệp (`thiserror` & `anyhow`)

#### 1. `thiserror`: Vũ khí cho tầng Domain / Core Logic / SDK

- **Mục đích sử dụng:**
  - Dành cho tầng **Domain Logic, Thư viện, SDK, Service Core** — nơi mà bên gọi (Caller / UI) **cần phân biệt chính xác loại lỗi** bằng `match` hoặc `if let` để xử lý theo từng kịch bản nghiệp vụ (ví dụ: thiếu tiền thì hiện nút nạp thêm, hết hàng thì báo liên hệ kho).
  - Vì vậy, lỗi ở tầng này bắt buộc phải là **Enum**.
- **Giải phóng Boilerplate:**
  - Thay vì tự viết tay hàng chục dòng `impl Display`, `impl std::error::Error`, `impl From<T>`, `thiserror` sử dụng derive macro sinh code 100% tự động lúc compile, không tốn chút chi phí runtime nào (zero-cost).

#### 2. Các Macro Cốt lõi của `thiserror`

- **`#[derive(thiserror::Error, Debug)]`:** Kích hoạt tính năng tự sinh trait `Error` và `Display`.
- **`#[error("...")]`:**
  - Tự động sinh `impl Display`.
  - Hỗ trợ nội suy vị trí: `{0}`, `{1}` đối với tuple enum.
  - Hỗ trợ nội suy tên trường: `{field_name}` đối với struct enum.
- **`#[from]`:**
  - Tự động sinh `impl From<SourceError>` cho biến thể đó.
  - Cho phép dùng toán tử `?` để tự động convert lỗi ngoại vi (như `std::io::Error`, `std::num::ParseIntError`) vào enum nghiệp vụ mà không cần gọi `.map_err()` thủ công.

#### 3. Các kinh nghiệm thực chiến đúc kết từ Buổi 8.1

- **Khai báo Enum lỗi chuẩn mực:**

  ```rust
  #[derive(thiserror::Error, Debug)]
  pub enum OrderError {
      #[error("Số lượng đặt hàng không hợp lệ: {0}")]
      InvalidQuantity(u32),

      #[error("Sản phẩm '{item_name}' hết hàng (còn {available})")]
      OutOfStock { item_name: String, available: u32 },

      #[error("Lỗi I/O: {0}")]
      Io(#[from] std::io::Error),

      #[error("Lỗi parse số: {0}")]
      Parse(#[from] std::num::ParseIntError),
  }
  ```

- **Pattern Matching có chủ đích:**
  Caller có thể dùng `if let Err(OrderError::InsufficientFunds { .. }) = result` để bắt đúng case cần đưa ra chỉ dẫn giao diện (UI Hint) cho người dùng.

#### 4. `anyhow`: Vũ khí cho tầng Ứng dụng / Controller / Tauri Command

- **Mục đích sử dụng:**
  - Dành cho tầng **Application, API Handlers, CLI Commands, Main function** — nơi mà lập trình viên chỉ cần **gom tất cả mọi loại lỗi lại** để ghi log hoặc trả về response cho frontend/user mà không muốn định nghĩa enum cồng kềnh.
- **Gom lỗi tự động bằng toán tử `?`:**
  - `anyhow::Result<T>` là bí danh kiểu (type alias) cho `Result<T, anyhow::Error>`.
  - Mọi kiểu lỗi cài đặt `std::error::Error` (kể cả enum do `thiserror` tạo ra hay lỗi từ thư viện `std`) đều được tự động chuyển đổi sang `anyhow::Error` khi dùng `?`.

#### 5. Macro `bail!`, `ensure!` và Chuỗi ngữ cảnh (Context Chaining)

- **`ensure!(condition, "...")`:**
  - Kiểm tra điều kiện đầu vào. Nếu điều kiện `true` thì cho qua; nếu `false` thì lập tức ngắt hàm và ném lỗi `anyhow`.
  - Tương đương: `if !condition { bail!("..."); }`.
- **`bail!("...")`:**
  - Ném lỗi tức thời mà không cần tạo enum (giống như `throw new Error(...)` trong JS/TS).
- **Gắn ngữ cảnh với `.context()` và `.with_context()`:**
  - `.context("Thông điệp tĩnh")`: Bổ sung thêm một lớp ngữ cảnh giải thích tại sao bước này thất bại.
  - `.with_context(|| format!("..."))`: Đóng gói ngữ cảnh bằng closure (chỉ tốn chi phí format chuỗi KHI CÓ LỖI XẢY RA, tối ưu CPU/RAM).
- **In chuỗi lỗi nhiều tầng:**
  - `println!("{err}");`: Chỉ in lỗi lớp ngoài cùng.
  - `println!("{err:#}");`: In toàn bộ chuỗi nguyên nhân (_Error Chain_) từ ngoài vào trong: `Lỗi cấp cao: Lỗi trung gian: Lỗi gốc OS`.
  - `println!("{err:?}");`: In toàn bộ cây lỗi chi tiết kèm stacktrace.

#### 6. Các kinh nghiệm thực chiến đúc kết từ Buổi 8.2

- **Mô hình kiến trúc chuẩn kết hợp `thiserror` + `anyhow`:**
  ```text
  [Tauri UI / API Controller / Main]  <--- Dùng `anyhow` (Gom lỗi, gắn .context, ensure!, in {err:#})
                  │
                  ▼ (Gọi hàm)
  [Domain Service / Core Logic / Lib] <--- Dùng `thiserror` (Tạo enum lỗi, phân loại chi tiết)
  ```
- **Hàm command điều phối ngắn gọn:**
  ```rust
  fn handle_checkout_command(...) -> anyhow::Result<()> {
      load_user_profile(path).context("Bước 1: Tải thông tin người dùng thất bại")?;
      checkout_cart(id, amount, balance).context("Bước 2: Xử lý giao dịch giỏ hàng thất bại")?;
      Ok(())
  }
  ```

---

### Bài 9: Asynchronous Programming với Tokio Runtime

#### 1. Tư duy Async Rust: Pull-based (Lazy) Future vs JS Eager Promise

| Đặc tính                      | JavaScript `Promise`                                                          | Rust `Future`                                                                                                               |
| :---------------------------- | :---------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| **Bản chất**                  | **Eager (Chạy ngay):** Khởi tạo là chạy ngầm ngay lập tức, không cần `await`. | **Pull-based / Lazy:** Gọi hàm async chỉ tạo ra State Machine, **hoàn toàn chưa chạy** cho đến khi được `.await` hoặc poll. |
| **Cơ chế**                    | Chạy trên Event Loop của V8/Node, tốn Heap và GC.                             | Không tốn Heap allocation mặc định (Zero-cost state machine).                                                               |
| **Hủy tác vụ (Cancellation)** | Phức tạp (cần `AbortController`).                                             | **Miễn phí:** Chỉ cần `drop(future)` là dừng ngay, dọn dẹp theo RAII.                                                       |

#### 2. Cơ chế Hoạt động của `.await` & Tokio Event Loop

- **`#[tokio::main]`:** Macro biến hàm `main` thành một Multi-threaded Work-stealing Thread Pool.
- **Điểm dừng không nghẽn (`.await`):** Khi gặp `.await`, nếu tác vụ I/O chưa xong, task sẽ nhường (yield) thread CPU cho các task khác chạy. Khi có tín hiệu sẵn sàng, Tokio đánh thức task dậy chạy tiếp từ đúng điểm dừng.
- **Quy tắc vàng:** **TUYỆT ĐỐI KHÔNG** gọi hàm blocking đồng bộ như `std::thread::sleep` trong async fn (sẽ làm đông cứng cả worker thread của Tokio!). Phải dùng `tokio::time::sleep`.

#### 3. Chạy Đồng Thời Nhiều Future với `tokio::join!`

- `tokio::join!(fut1, fut2, fut3)` tương đương `Promise.all` của JS:
  - Cho phép chạy đồng thời nhiều future cùng lúc trên cùng một context.
  - Tổng thời gian thực thi xấp xỉ bằng thời gian của **tác vụ lâu nhất** (thay vì cộng dồn tuần tự).

#### 4. Các kinh nghiệm thực chiến đúc kết từ Buổi 9.1

- **Đo thời gian với `Instant`:**
  - `duration:?`: In ra chuỗi có sẵn đơn vị (ví dụ `154.4184ms`). Không cần thêm chữ `ms` vào sau chuỗi format nếu đã dùng debug format `:?`.
  - `duration.as_millis()`: Lấy số nguyên mili-giây dạng `u128`.
- **Kết hợp `tokio::join!` với toán tử `?` của `anyhow`:**
  ```rust
  let (res1, res2) = tokio::join!(task_1(), task_2());
  let val1 = res1?;
  let val2 = res2?;
  ```

#### 5. `tokio::spawn` vs `std::thread::spawn` (Green Tasks vs OS Threads)

| Tiêu chí             | `std::thread::spawn` (OS Thread)                             | `tokio::spawn` (Async Green Task)                                                                                            |
| :------------------- | :----------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| **Bản chất**         | Kernel Thread do Hệ điều hành quản lý.                       | **Green Thread (Task)** do Tokio quản lý ở User Space.                                                                       |
| **Kích thước Stack** | **~1MB - 2MB** mỗi thread.                                   | **Chỉ vài trăm Bytes** mỗi task!                                                                                             |
| **Context Switch**   | Chuyển đổi qua lại giữa User Mode và Kernel Mode $\to$ Nặng. | Chuyển đổi nội bộ trong user space $\to$ Siêu nhẹ.                                                                           |
| **Quy mô (Scale)**   | Vài ngàn thread là sập RAM hoặc chậm hệ thống.               | **Hàng chục ngàn đến 100.000+ tasks** chạy đồng thời mượt mà.                                                                |
| **Mô hình thực thi** | 1 Thread = 1 OS Core Thread.                                 | **M:N Multi-threading:** Hàng trăm ngàn task (M) phân phối trên vài OS Worker Threads (N) bằng thuật toán **Work-Stealing**. |

#### 6. Ràng buộc `T: Send + 'static` trong Async Tasks

- **`T: Send`:** Vì Tokio có cơ chế Work-Stealing (chuyển task từ thread CPU này sang thread CPU khác khi rảnh việc), kiểu dữ liệu truyền vào task bắt buộc phải là `Send`.
- **`T: 'static`:** Task chạy nền có thể sống lâu hơn scope của hàm cha sinh ra nó. Compiler cấm tiệt việc mượn tham chiếu ngắn hạn (`&'a str`) từ stack của hàm cha vào task:
  ```rust
  let local = String::from("cục bộ");
  let slice = &local;
  tokio::spawn(async move { println!("{slice}"); }); // LỖI: `local` does not live long enough
  ```
- **Chia sẻ dữ liệu vào nhiều tasks:** Tiếp tục dùng công thức chuẩn mực **`Arc<T>`**!

#### 7. Các kinh nghiệm thực chiến đúc kết từ Buổi 9.2

- **Hiệu năng xử lý đồng thời siêu khủng:**
  1.000 tasks (mỗi task trễ 50-99ms), nếu chạy tuần tự mất $\approx 75$ giây. Nhưng chạy đồng thời qua `tokio::spawn` chỉ mất $\approx 115\text{ms}$!
- **`JoinHandle` của Tokio là một Future:**
  Kết quả trả về của `tokio::spawn` có thể `.await` không hề làm nghẽn OS thread:
  ```rust
  let result = handle.await.unwrap();
  ```

#### 8. `tokio::sync::Mutex` vs `std::sync::Mutex` (Ranh giới `.await`)

| Tiêu chí                  | `std::sync::Mutex` (Đồng bộ)                                           | `tokio::sync::Mutex` (Bất đồng bộ)                             |
| :------------------------ | :--------------------------------------------------------------------- | :------------------------------------------------------------- |
| **Cơ chế khóa**           | Khóa chặn OS Thread (**Blocking**).                                    | Khóa nhường luồng (**Non-blocking, awaitable**).               |
| **Hiệu năng**             | Cực nhanh, zero-overhead cho RAM.                                      | Chậm hơn 1 chút do quản lý Future state.                       |
| **Giữ khóa qua `.await`** | **CẤM:** Gây lỗi compile hoặc đóng băng toàn bộ thread pool của Tokio! | **ĐƯỢC:** Thiết kế riêng để an toàn khi giữ lock qua `.await`. |
| **Khi nào nên dùng?**     | Cập nhật dữ liệu in-memory nhanh rồi nhả khóa ngay trước `.await`.     | Bắt buộc phải giữ khóa trong lúc gọi async network/I/O.        |

#### 9. Giải cứu Event Loop với `tokio::task::spawn_blocking`

- **Vấn đề CPU-Bound:** Các hàm nặng CPU (như băm mật khẩu Argon2, mã hóa AES, tính toán toán học lớn) hoặc I/O đồng bộ (`std::fs`) nếu chạy trong async fn sẽ chiếm dụng 100% CPU của worker thread, làm đơ Event Loop và giật lag UI Tauri.
- **Giải pháp `spawn_blocking`:**
  - Tokio cung cấp một Thread Pool riêng (tự co giãn đến 512 OS threads) chuyên trị các tác vụ blocking.
  - Cú pháp:
    ```rust
    let result = tokio::task::spawn_blocking(move || {
        heavy_cpu_computation()
    }).await?;
    ```
  - Trong lúc thread pool blocking làm việc, các worker thread async chính vẫn phản hồi mượt mà từng mili-giây.

#### 10. Các kinh nghiệm thực chiến đúc kết từ Buổi 9.3

- **Hành vi tuần tự khi giữ lock qua `.await`:**
  Nếu một hàm async giữ `tokio::sync::Mutex` trong suốt thời gian sleep/network, các task khác cùng xin khóa sẽ phải xếp hàng chờ đợi lần lượt, bảo vệ trạng thái tuyệt đối an toàn.
- **Phối hợp `tokio::join!` giữa Ticker và Blocking Task:**
  Cho phép chạy song song một async UI task nhẹ nhàng cùng lúc với một blocking CPU computation mà không hề có hiện tượng nghẽn luồng.
