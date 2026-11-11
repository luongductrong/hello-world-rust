# Hướng dẫn chi tiết về Kiểu dữ liệu (Data Types) trong Rust

Tài liệu này giúp bạn nắm rõ ý nghĩa **con số bit** đằng sau các kiểu dữ liệu (`i32`, `u64`, `f64`...), cách Rust xử lý **tràn số (overflow)**, và quy tắc thực tế để chọn kiểu dữ liệu mà **không lo tốn bộ nhớ hay tràn số**.

---

## 1. Giải đáp nhanh 2 thắc mắc cốt lõi

### Con số đằng sau `i`, `u`, `f` có phải là số bit không?
**Chính xác 100%.** Con số đứng sau là **tổng số bit** mà kiểu dữ liệu đó chiếm trong bộ nhớ (nhớ quy đổi: **1 byte = 8 bit**).

- **`i`** (*signed integer* — số nguyên **có dấu**): lưu được cả số âm, `0` và số dương. Vì mất **1 bit làm dấu** (theo chuẩn bù 2 / *two's complement*), phạm vi của `iN` là từ $-2^{N-1}$ đến $2^{N-1} - 1$.
- **`u`** (*unsigned integer* — số nguyên **không dấu**): chỉ lưu số $\ge 0$. Dùng toàn bộ $N$ bit cho giá trị nên phạm vi lớn gấp đôi về phía dương: từ $0$ đến $2^N - 1$.
- **`f`** (*floating-point* — số thực dấu phẩy động chuẩn IEEE 754): số bit càng lớn thì **độ chính xác (số chữ số có nghĩa)** và biên độ số càng cao.

### Có cần sợ "đặt quá nhỏ sẽ tràn số, đặt quá lớn sẽ tốn bộ nhớ" không?
Tóm tắt ngắn gọn:
1. **Đừng quá lo chuyện tốn bộ nhớ với biến đơn lẻ:** Một biến `i64` tốn 8 bytes, `i32` tốn 4 bytes — chênh nhau **4 bytes** là con số gần như bằng 0 so với RAM máy tính (hàng GB). Thậm chí CPU 64-bit xử lý `i32` và `i64` đều rất nhanh. Bạn chỉ cần bóp nhỏ kiểu dữ liệu (`u8`, `u16`, `f32`...) khi lưu **mảng hàng triệu phần tử**, dữ liệu đồ họa/AI, hoặc giao thức nhị phân.
2. **Tràn số trong Rust an toàn hơn C và Java rất nhiều:** Rust không âm thầm gây lỗi bộ nhớ như C hay âm thầm quay vòng không báo như Java ở chế độ dev. Compiler và runtime của Rust có cơ chế bắt tràn số và có các hàm xử lý tràn số rõ ràng (chi tiết ở Mục 3).

---

## 2. Bảng tra cứu các kiểu số (Numeric Types)

### 2.1. Số nguyên có dấu (`i`)

| Kiểu | Số bit | Số byte | Giá trị nhỏ nhất (`MIN`) | Giá trị lớn nhất (`MAX`) | Dễ nhớ trong thực tế | Tương đương C / Java / TS |
| :--- | :---: | :---: | :--- | :--- | :--- | :--- |
| **`i8`** | 8 | 1 | `-128` | `127` | Khoảng ±1 trăm | `int8_t` (C) / `byte` (Java) |
| **`i16`** | 16 | 2 | `-32_768` | `32_767` | Khoảng ±32 nghìn | `int16_t` (C) / `short` (Java) |
| **`i32`** *(mặc định)* | 32 | 4 | `-2_147_483_648` | `2_147_483_647` | **Khoảng ±2.14 tỷ** | `int32_t` (C) / `int` (Java) |
| **`i64`** | 64 | 8 | `-9_223_372_036_854_775_808` | `9_223_372_036_854_775_807` | **Khoảng ±9.22 tỷ tỷ** ($\approx 9.22 \times 10^{18}$) | `int64_t` (C) / `long` (Java) |
| **`i128`** | 128 | 16 | $\approx -1.7 \times 10^{38}$ | $\approx +1.7 \times 10^{38}$ | Cực lớn (39 chữ số) | `__int128` (GCC) |
| **`isize`** | 32 hoặc 64 | 4 hoặc 8 | Phụ thuộc kiến trúc CPU (64-bit giống `i64`) | Phụ thuộc kiến trúc CPU | Hiệu số con trỏ / offset | `ptrdiff_t` / `ssize_t` (C) |

> **So sánh hay với JavaScript/TypeScript:** Kiểu `number` trong JS là số thực `f64`, chỉ biểu diễn **chính xác từng đơn vị** số nguyên tới $2^{53} - 1 \approx 9\text{ triệu tỷ}$ (`Number.MAX_SAFE_INTEGER`). Trong khi đó, `i64` của Rust lưu chính xác số nguyên tới hơn **9.22 tỷ tỷ** (gấp ~1.024 lần!).

---

### 2.2. Số nguyên không dấu (`u`)

| Kiểu | Số bit | Số byte | Giá trị (`MIN` → `MAX`) | Dễ nhớ trong thực tế | Thường dùng làm gì? |
| :--- | :---: | :---: | :--- | :--- | :--- |
| **`u8`** | 8 | 1 | `0` → `255` | 1 byte thô | Ký tự ASCII, màu RGB (`0..=255`), dữ liệu nhị phân (`Vec<u8>`) |
| **`u16`** | 16 | 2 | `0` → `65_535` | Khoảng 65 nghìn | Cổng mạng (Port `0..=65535`), dữ liệu âm thanh 16-bit |
| **`u32`** | 32 | 4 | `0` → `4_294_967_295` | **Khoảng 4.29 tỷ** | Ký tự Unicode (code point), kích thước vừa, đếm không âm |
| **`u64`** | 64 | 8 | `0` → `18_446_744_073_709_551_615` | **Khoảng 18.4 tỷ tỷ** | ID tự tăng lớn, timestamp nano-giây, kích thước file, hash |
| **`u128`** | 128 | 16 | `0` → `340_282_366_920_938_463_463_374_607_431_768_211_455` | Khoảng $3.4 \times 10^{38}$ | UUID (128-bit), thuật toán mật mã, tính toán số lớn |
| **`usize`** | 32 hoặc 64 | 4 hoặc 8 | Trên máy 64-bit: giống `u64` | Kích thước vùng nhớ | **Index mảng/Vec (`arr[i]`)**, `.len()`, kích thước bộ nhớ (`size_t` ở C) |

> **Lưu ý đặc biệt về `usize`:** Trong Rust, chỉ số mảng (index) **bắt buộc phải là `usize`** (`arr[0usize]`), không dùng `i32` hay `u32` trực tiếp để index như C/Java/JS được. Lý do là `usize` luôn vừa khớp kích thước địa chỉ bộ nhớ của CPU (4 bytes trên hệ 32-bit, 8 bytes trên hệ 64-bit).

---

### 2.3. Số thực (`f32` và `f64`)

| Kiểu | Số bit | Số byte | Độ chính xác | Phạm vi giá trị | Tương đương C / Java / TS |
| :--- | :---: | :---: | :--- | :--- | :--- |
| **`f32`** | 32 | 4 | ~6–7 chữ số thập phân có nghĩa | $\pm 3.4 \times 10^{38}$ | `float` (C / Java) |
| **`f64`** *(mặc định)* | 64 | 8 | ~15–17 chữ số thập phân có nghĩa | $\pm 1.79 \times 10^{308}$ | `double` (C / Java), `number` (JS/TS) |

- **Mặc định dùng `f64`:** Trên CPU 64-bit hiện đại, tốc độ tính toán `f64` gần tương đương `f32` nhưng độ chính xác cao hơn hẳn.
- **Khi nào dùng `f32`?** Game 3D, đồ họa (GPU ưu tiên `f32`), hoặc AI/Machine Learning (hàng triệu trọng số — giảm một nửa RAM/VRAM).
- **Cảnh báo chung cho `f32` và `f64`:** Vì lưu theo nhị phân chuẩn IEEE 754, cả hai đều có sai số làm tròn (`0.1 + 0.2 == 0.30000000000000004` giống hệt JS/Java/C). **Không dùng `f32`/`f64` cho tiền tệ** đòi hỏi chính xác tuyệt đối — hãy lưu bằng số nguyên ở đơn vị nhỏ nhất (như `i64` tính bằng đồng hoặc cent) hoặc dùng crate chuyên dụng như `rust_decimal`.

---

### 2.4. Mẹo viết số trong Rust cho dễ đọc & rõ kiểu

Rust cho phép dùng dấu gạch dưới `_` ở bất kỳ đâu trong con số để phân tách cho dễ nhìn, và gắn hậu tố kiểu trực tiếp vào cuối số:

```rust
let budget: i64 = 1_500_000_000; // 1.5 tỷ, cực kỳ dễ đọc
let port = 8080u16;               // Hậu tố kiểu: port tự có kiểu u16
let fee = 10_000_i64;             // Có thể thêm _ trước tên kiểu
let hex = 0xff_ec_de_5e_u32;      // Hệ 16 (Hex)
let bin = 0b1111_0000u8;          // Hệ 2 (Binary)
let oct = 0o755u16;               // Hệ 8 (Octal)
let byte_char = b'A';             // Byte literal, tự có kiểu u8 (= 65)
```

---

## 3. Tràn số (Integer Overflow) trong Rust hoạt động thế nào?

Giả sử bạn dùng `u8` (tối đa `255`) và cộng thêm để kết quả thành `256`:

1. **Khi gán số cố định vượt giới hạn:** Compiler báo lỗi **ngay lúc biên dịch**, không cho build.
   ```rust
   let x: u8 = 300; // Lỗi compile: literal out of range for `u8`
   ```
2. **Lúc chạy ở chế độ Debug** (`cargo run`, `cargo test`):
   - Nếu phép tính bị tràn lúc chạy (runtime), Rust sẽ **panic (dừng chương trình ngay lập tức)** và báo rõ dòng lỗi `attempt to add with overflow`.
   - Điều này giúp bạn phát hiện bug sớm khi lập trình, thay vì âm thầm tính sai như Java hay gây lỗi bộ nhớ như C.
3. **Lúc chạy ở chế độ Release** (`cargo build --release`, `cargo run --release`):
   - Để đạt hiệu năng tối đa (không tốn lệnh kiểm tra tràn số sau mỗi phép cộng), Rust **không panic** mà thực hiện **quay vòng (two's complement wrapping)**: `255u8 + 1` sẽ quay về `0`, `127i8 + 1` sẽ quay về `-128`.

### Bẫy hay gặp nhất với người mới: "Underflow" ở kiểu `u`
Đừng chọn `u32` hay `u64` chỉ vì nghĩ *"số này trong đời thực không âm"*, nếu có lúc bạn làm phép **trừ**:

```rust
let apples: u32 = 5;
let eaten: u32 = 10;
let left = apples - eaten; // PANIC ở debug! (hoặc thành 4_294_967_291 ở release!)
```
Nếu có khả năng trừ âm, dùng `i32` / `i64` hoặc dùng các phương thức an toàn dưới đây.

### 4 nhóm phương thức kiểm soát tràn số tường minh trong Rust
Nếu một phép tính **có nguy cơ tràn số** (kể cả ở bản release), đừng dùng `+`, `-`, `*` thông thường mà hãy dùng 1 trong 4 họ hàm có sẵn trên mọi kiểu số nguyên:

| Họ hàm | Hành vi khi bị tràn | Kiểu trả về | Ví dụ thực tế |
| :--- | :--- | :--- | :--- |
| **`checked_*`** (`checked_add`, `checked_sub`...) | Trả về `None` nếu tràn, `Some(val)` nếu hợp lệ | `Option<T>` | Tính tiền / cộng điểm nhập từ user, cần báo lỗi nếu quá lớn |
| **`saturating_*`** (`saturating_add`, `saturating_sub`...) | Đụng trần/sàn thì **giữ nguyên ở `MAX` hoặc `MIN`** | `T` | Máu nhân vật game: `hp.saturating_sub(damage)` tối đa về `0`, không âm |
| **`wrapping_*`** (`wrapping_add`, `wrapping_mul`...) | Cố tình cho quay vòng (cả debug lẫn release đều không panic) | `T` | Thuật toán hash, mã hóa, đồ họa bit |
| **`overflowing_*`** (`overflowing_add`...) | Trả về giá trị đã quay vòng **kèm cờ báo có tràn hay không** | `(T, bool)` | Cần biết phép toán vừa tràn bit nhớ |

```rust
let hp: u8 = 20;
let hp_after_hit = hp.saturating_sub(50); // = 0 (không panic!)

let max_u8: u8 = 250;
match max_u8.checked_add(10) {
    Some(val) => println!("Kết quả: {val}"),
    None => println!("Bị tràn số rồi!"), // Chạy nhánh này
}
```

---

## 4. Giải tỏa nỗi lo "Tốn bộ nhớ": Khi nào chọn kiểu nhỏ, khi nào chọn kiểu lớn?

### Sự thật về RAM và hiệu năng
- **Biến cục bộ / Tham số hàm / Trường dữ liệu thông thường:** Chênh lệch giữa `u8` (1 byte) và `i64` (8 bytes) là **7 bytes**. Với RAM máy tính hiện nay (tính bằng tỷ bytes), việc tiết kiệm 7 bytes ở vài chục biến là **vô nghĩa**, thậm chí có thể khiến code phải ép kiểu qua lại rườm rà.
- **Hiện tượng Alignment & Padding (Căn lề bộ nhớ):** CPU đọc dữ liệu hiệu quả nhất theo bội số của 4 hoặc 8 bytes. Nếu bạn tạo một `struct` có 1 trường `u8` và 1 trường `u64`, tổng kích thước struct đó vẫn là **16 bytes** (Rust tự chèn 7 bytes đệm trống), chứ không phải 9 bytes! Vì thế dùng `u8` đơn lẻ chưa chắc tiết kiệm được byte nào.

### Khi nào MỚI thực sự cần bóp nhỏ kiểu dữ liệu?
1. **Dữ liệu lặp lại với số lượng cực lớn:** Ví dụ một mảng 10 triệu phần tử (`Vec<u8>` tốn ~10 MB, còn `Vec<i64>` tốn ~80 MB — lúc này chênh nhau tới 70 MB và ảnh hưởng rõ rệt đến CPU cache!).
2. **Đọc/ghi file nhị phân, gói tin mạng, phần cứng nhúng (Embedded):** Phải khớp từng bit theo đặc tả (header 16-bit dùng `u16`, IPv4 dùng `[u8; 4]`).

### Bảng chọn kiểu thực chiến (Cheat Sheet)

| Nhu cầu của bạn | Chọn kiểu nào? | Lý do |
| :--- | :--- | :--- |
| Số nguyên bình thường (tuổi, số lượng nhỏ, tính toán chung) | **`i32`** | Kiểu mặc định của Rust, phạm vi ±2.1 tỷ là quá dư cho đa số trường hợp |
| Số có thể vượt 2 tỷ (tiền VNĐ tổng lớn, timestamp, dung lượng lớn, ID) | **`i64`** (hoặc **`u64`**) | An toàn tới ~9.2 tỷ tỷ, chạy tự nhiên trên CPU 64-bit |
| Index mảng, độ dài danh sách, vòng lặp theo kích thước | **`usize`** | Chuẩn bắt buộc của Rust để đánh chỉ mục slice/Vec/array |
| Số thực (điểm số, tọa độ, phép đo, tính toán khoa học) | **`f64`** | Mặc định của Rust, chính xác gấp đôi `f32`, tốc độ tương đương |
| Đồ họa 3D, vector game, trọng số AI cần tiết kiệm 50% RAM | **`f32`** | Nhẹ gấp đôi `f64`, tương thích tốt với GPU / SIMD |
| Dữ liệu thô từng byte, màu sắc `0..=255`, ký tự ASCII | **`u8`** | 1 byte gọn nhẹ, chuẩn xử lý buffer/stream |
| Cổng mạng (Port), năm/ID nhỏ trong cấu trúc lớn | **`u16`** | Vừa khít `0..=65535` |
| Giá trị đúng/sai | **`bool`** | Chỉ có `true` hoặc `false` (1 byte) |

---

## 5. Ép kiểu dữ liệu (Type Casting & Conversion)

Khác với JS (tự ép kiểu lung tung), hay C/Java (tự mở rộng `int` lên `long`/`double`), **Rust không bao giờ tự ép kiểu ngầm** giữa các kiểu số khác nhau. Kể cả cộng `i32` với `i64` cũng bị compiler từ chối!

Có 3 cách chuyển kiểu trong Rust, xếp theo mức độ an toàn từ cao xuống thấp:

### Cách 1: `From` / `Into` — An toàn tuyệt đối (từ nhỏ → lớn)
Chỉ biên dịch được khi chắc chắn **không bao giờ mất dữ liệu**:
```rust
let a: i32 = 100;
let b: i64 = i64::from(a); // Hoặc: let b: i64 = a.into();
// i32::from(b) -> LỖI COMPILE! Vì i64 có thể lớn hơn i32.
```

### Cách 2: `TryFrom` / `TryInto` — An toàn có kiểm tra (từ lớn → nhỏ hoặc `i` ↔ `u`)
Trả về `Result` để bạn xử lý nếu giá trị bị tràn:
```rust
let big: i64 = 50_000;
let small: Result<i16, _> = i16::try_from(big); // Err(...) vì 50_000 > 32_767
```

### Cách 3: Từ khóa `as` — Ép thẳng tay (cẩn thận mất dữ liệu!)
Hoạt động giống `(int)` trong C/Java: **không panic, nhưng có thể cắt cụt bit hoặc làm mất phần thập phân**:
```rust
let pi: f64 = 3.99;
let truncated = pi as i32; // = 3 (mất phần thập phân)

let large: i32 = 300;
let byte = large as u8;    // = 44 (vì 300 % 256 = 44, âm thầm mất dữ liệu!)
```
> **Nguyên tắc:** Chỉ dùng `as` khi bạn **chắc chắn giá trị nằm gọn trong khoảng đích** (ví dụ `let x: usize = 5; x as u64`), hoặc cố tình muốn cắt bit. Nếu chuyển dữ liệu từ nguồn ngoài/user, hãy ưu tiên `try_from` / `try_into`.

---

## 6. Tổng hợp nhanh các kiểu dữ liệu còn lại của Rust

Ngoài các kiểu số ở trên, bức tranh đầy đủ về kiểu dữ liệu cơ bản của Rust gồm:

### 6.1. Kiểu vô hướng (Scalar) khác
- **`bool`** (`1 byte`): chỉ nhận `true` hoặc `false`. Rust không coi `0`/`1` hay chuỗi rỗng là boolean.
- **`char`** (`4 bytes`): lưu **1 Unicode Scalar Value** trong cặp nháy đơn (`'a'`, `'ế'`, `'🦀'`).
  - *Chú ý:* `char` trong C là 1 byte, trong Java là 2 bytes (UTF-16), còn trong Rust luôn là **4 bytes** (UTF-32 cho từng ký tự đơn lẻ) nên chứa được mọi emoji và ký tự tiếng Việt có dấu trong 1 `char`.

### 6.2. Kiểu gộp (Compound) — Kích thước cố định trên Stack
- **Tuple `(T1, T2, ...)`:** Nhóm nhiều giá trị có thể **khác kiểu**:
  ```rust
  let user: (&str, u8, bool) = ("An", 25, true);
  let name = user.0;          // Truy cập bằng dấu chấm + chỉ số
  let (n, age, active) = user; // Destructuring
  ```
  - **Unit type `()`:** Tuple rỗng `()`, chiếm **0 byte** trong bộ nhớ! Đây là kiểu trả về mặc định của các hàm không trả về giá trị (tương đương `void` trong Java/C).
- **Array `[T; N]`:** Mảng có **độ dài cố định ngay lúc biên dịch**, mọi phần tử cùng kiểu `T`, nằm trên Stack:
  ```rust
  let rgb: [u8; 3] = [255, 128, 0]; // Tốn đúng 3 * 1 = 3 bytes
  let zeros = [0i32; 5];            // Mảng 5 phần tử toàn số 0 (20 bytes)
  ```

### 6.3. Kiểu dữ liệu động (Lưu trên Heap — học sâu ở Chặng 2–4)
- **`Vec<T>`:** Mảng co giãn độ dài lúc chạy (tương đương `Array` ở JS / `ArrayList<T>` ở Java).
- **Chuỗi ký tự (`String` và `&str`):**
  - `&str`: lát cắt chuỗi mượn (*string slice*), thường dùng cho chuỗi cố định `"hello"`.
  - `String`: chuỗi sở hữu trên Heap, thêm bớt ký tự được.
  - Cả hai đều được mã hóa **UTF-8** (mỗi ký tự tiếng Anh tốn 1 byte, ký tự tiếng Việt có dấu tốn 2–3 bytes, emoji tốn 4 bytes — nên trong Rust **không thể** lấy ký tự bằng `s[0]` như mảng!).

---

## 7. Mẹo tự kiểm chứng kích thước & giới hạn bằng code Rust

Bất cứ lúc nào quên một kiểu tốn bao nhiêu byte hay giá trị Min/Max là bao nhiêu, bạn có thể gõ nhanh đoạn này vào Rust để tự kiểm tra:

```rust
use std::mem::size_of;

fn check_types() {
    println!("i32: {} bytes | MIN = {} | MAX = {}", size_of::<i32>(), i32::MIN, i32::MAX);
    println!("i64: {} bytes | MIN = {} | MAX = {}", size_of::<i64>(), i64::MIN, i64::MAX);
    println!("usize: {} bytes | MAX = {}", size_of::<usize>(), usize::MAX);
    println!("char: {} bytes | bool: {} byte | (): {} bytes",
        size_of::<char>(), size_of::<bool>(), size_of::<()>()
    );
}
```
