# Cẩm nang toàn diện về Toán tử & Phép tính trong Rust (Operators & Expressions)

Tài liệu này tổng hợp đầy đủ các **toán tử, phép tính, so sánh, logic, thao tác bit và các toán tử đặc trưng** trong Rust. Đồng thời đối chiếu trực tiếp với thói quen từ **JavaScript / TypeScript, Java và C** để giúp bạn tránh các lỗi biên dịch thường gặp.

---

## Mục lục

1. [Bảng đối chiếu "Cai nghiện": Những toán tử KHÔNG TỒN TẠI trong Rust](#1-bảng-đối-chiếu-cai-nghiện-những-toán-tử-không-tồn-tại-trong-rust)
2. [Toán tử số học (Arithmetic Operators)](#2-toán-tử-số-học-arithmetic-operators)
3. [Toán tử gán & Gán kết hợp (Assignment Operators)](#3-toán-tử-gán--gán-kết-hợp-assignment-operators)
4. [Toán tử so sánh (Comparison Operators)](#4-toán-tử-so-sánh-comparison-operators)
5. [Toán tử logic & Phủ định (Logical Operators)](#5-toán-tử-logic--phủ-định-logical-operators)
6. [Toán tử thao tác bit (Bitwise Operators)](#6-toán-tử-thao-tác-bit-bitwise-operators)
7. [Toán tử dải giá trị (Range Operators)](#7-toán-tử-dải-giá-trị-range-operators)
8. [Toán tử tham chiếu & Giải tham chiếu (Borrow & Dereference)](#8-toán-tử-tham-chiếu--giải-tham-chiếu-borrow--dereference)
9. [Toán tử xử lý lỗi, Pattern Matching & Luồng](#9-toán-tử-xử-lý-lỗi-pattern-matching--luồng)
10. [Toán tử ép kiểu `as`](#10-toán-tử-ép-kiểu-as)
11. [Thứ tự ưu tiên của các toán tử (Operator Precedence)](#11-thứ-tự-ưu-tiên-của-các-toán-tử-operator-precedence)
12. [Nâng cao: Operator Overloading với `std::ops`](#12-nâng-cao-operator-overloading-với-stdops)

---

## 1. Bảng đối chiếu "Cai nghiện": Những toán tử KHÔNG TỒN TẠI trong Rust

Khi chuyển từ JS/TS, Java hay C sang Rust, bạn sẽ gặp lỗi biên dịch nếu gõ theo thói quen cũ. Hãy xem bảng thay thế sau:

| Thói quen ở JS / TS / Java / C   | Có trong Rust không? | Cách viết chuẩn trong Rust          | Lý do Rust thiết kế như vậy                                                               |
| :------------------------------- | :------------------: | :---------------------------------- | :---------------------------------------------------------------------------------------- |
| `i++` hoặc `++i`                 |      **Không**       | `i += 1;`                           | Tránh side-effect ngầm trong biểu thức, tránh mơ hồ thứ tự tính toán                      |
| `i--` hoặc `--i`                 |      **Không**       | `i -= 1;`                           | Tương tự như trên                                                                         |
| `a ? b : c` (toán tử 3 ngôi)     |      **Không**       | `if a { b } else { c }`             | Vì trong Rust `if/else` bản thân nó đã là một **expression** trả về giá trị               |
| `===` và `!==` (so sánh kiểu)    |      **Không**       | `==` và `!=`                        | Rust kiểm tra kiểu tĩnh lúc compile; hai vế khác kiểu sẽ **không bao giờ biên dịch được** |
| `~` (bitwise NOT đảo bit)        |      **Không**       | `!x` (dùng dấu `!`)                 | Rust tái sử dụng dấu `!` cho cả boolean NOT và bitwise NOT trên số nguyên                 |
| `**` (toán tử lũy thừa)          |      **Không**       | `x.pow(3)` hoặc `f.powi(3)`         | Lũy thừa được xử lý bằng method; ký tự `^` trong Rust là phép **XOR bit**                 |
| `?.` (optional chaining)         |      **Không**       | Toán tử `?`, `.and_then()`, `match` | Rust dùng `Option<T>` tường minh, không có giá trị `null` / `undefined` ngầm              |
| `??` (nullish coalescing)        |      **Không**       | `opt.unwrap_or(default)`            | Tương tự trên, xử lý qua phương thức của `Option<T>`                                      |
| `a = b = c = 0;` (gán liên hoàn) |      **Không**       | Khai báo/gán từng dòng              | Phép gán `=` trong Rust trả về `()` (unit type), không trả về giá trị vừa gán             |

---

## 2. Toán tử số học (Arithmetic Operators)

| Toán tử | Tên gọi       | Ví dụ                    | Kết quả                     | Ghi chú quan trọng                                                               |
| :-----: | :------------ | :----------------------- | :-------------------------- | :------------------------------------------------------------------------------- |
| **`+`** | Cộng          | `10 + 3`                 | `13`                        | Cùng kiểu số                                                                     |
| **`-`** | Trừ / Đổi dấu | `10 - 3`, `-x`           | `7`, `-x`                   | Chú ý underflow khi trừ số không dấu (`u8`, `u32`)                               |
| **`*`** | Nhân          | `10 * 3`                 | `30`                        |                                                                                  |
| **`/`** | Chia          | `10 / 3`<br>`10.0 / 3.0` | `3`<br>`3.3333333333333335` | Số nguyên chia số nguyên **luôn làm tròn về 0** (giống C/Java, khác JS ra float) |
| **`%`** | Chia lấy dư   | `10 % 3`<br>`-10 % 3`    | `1`<br>`-1`                 | Dấu của kết quả **cùng dấu với số bị chia** (số đứng trước)                      |

### 3 quy tắc "sống còn" khi làm phép tính trong Rust:

#### Quy tắc 1: Tuyệt đối KHÔNG ép kiểu ngầm (No Implicit Coercion)

Trong JS bạn có thể viết `5 + 2.5` ra `7.5`. Trong C/Java `int` tự mở rộng lên `double`. Nhưng **Rust bắt buộc 2 toán hạng phải tuyệt đối cùng kiểu**:

```rust
let a: i32 = 10;
let b: f64 = 2.5;

// let c = a + b;           //  LỖI COMPILE: cannot add `f64` to `i32`
let c = a as f64 + b;       //  Hợp lệ: ép a sang f64 rồi cộng
let d = a + (b as i32);     //  Hợp lệ: ép b sang i32 (mất phần thập phân) -> 10 + 2 = 12
```

#### Quy tắc 2: Phép chia cho 0

- **Số nguyên chia cho 0:**
  - Nếu hằng số cố định (`10 / 0`): compiler chặn ngay lúc compile (`attempt to divide by zero`).
  - Nếu qua biến lúc chạy runtime: chương trình sẽ **panic ngay lập tức**.
- **Số thực (`f32`, `f64`) chia cho 0:** Tuân theo chuẩn IEEE 754 giống JS/Java:
  ```rust
  let x = 1.0 / 0.0;  // = inf (vô cực dương)
  let y = -1.0 / 0.0; // = -inf (vô cực âm)
  let z = 0.0 / 0.0;  // = NaN (Not a Number)
  ```

#### Quy tắc 3: Lũy thừa không dùng `^` hay `**`

```rust
// let x = 2 ** 3;          //  LỖI COMPILE: Rust không có toán tử **
// let x = 2 ^ 3;           //  CẢNH BÁO: Đây là phép XOR bit (2 ^ 3 = 1), không phải 8!

let x = 2i32.pow(3);        //  = 8 (số nguyên mũ số nguyên dương u32)
let y = 2.0f64.powi(3);     //  = 8.0 (số thực mũ số nguyên)
let z = 2.0f64.powf(0.5);    //  = 1.4142... (căn bậc 2, số thực mũ số thực)
```

---

## 3. Toán tử gán & Gán kết hợp (Assignment Operators)

| Toán tử                       | Tương đương         | Yêu cầu                             |
| :---------------------------- | :------------------ | :---------------------------------- |
| `x = y`                       | Gán giá trị         | `x` phải khai báo với **`let mut`** |
| `x += y`                      | `x = x + y`         | `x` phải `mut` và cùng kiểu với `y` |
| `x -= y`                      | `x = x - y`         | `x` phải `mut`                      |
| `x *= y`                      | `x = x * y`         | `x` phải `mut`                      |
| `x /= y`                      | `x = x / y`         | `x` phải `mut`                      |
| `x %= y`                      | `x = x % y`         | `x` phải `mut`                      |
| `x &= y`, `x \|= y`, `x ^= y` | Thao tác bit và gán | Áp dụng cho số nguyên hoặc `bool`   |
| `x <<= y`, `x >>= y`          | Dịch bit và gán     | `x` phải `mut`                      |

### Bẫy tư duy từ C/Java/JS: Phép gán trong Rust trả về `()`

Trong C/JS, bạn có thể viết:

```javascript
// JS / C:
if ((val = getNext()) !== null) { ... }
a = b = c = 10;
```

Trong **Rust, phép gán trả về kiểu unit `()` (rỗng)**, nên những đoạn code trên là **bất hợp pháp**:

```rust
let mut x = 5;
// if (x = 10) { }   //  LỖI COMPILE ngay lập tức!
// Kiểu trả về là () chứ không phải bool.
// Điều này ngăn chặn 100% lỗi kinh điển gõ nhầm `=` thay vì `==`!
```

---

## 4. Toán tử so sánh (Comparison Operators)

Mọi toán tử so sánh đều trả về kiểu **`bool`** (`true` hoặc `false`).

| Toán tử  | Ý nghĩa           | Ví dụ                        |                  Kết quả                   |
| :------: | :---------------- | :--------------------------- | :----------------------------------------: |
| **`==`** | Bằng nhau         | `5 == 5`<br>`"abc" == "abc"` | `true`<br>`true` (so sánh nội dung chuỗi!) |
| **`!=`** | Khác nhau         | `5 != 3`                     |                   `true`                   |
| **`<`**  | Nhỏ hơn           | `3 < 5`                      |                   `true`                   |
| **`>`**  | Lớn hơn           | `5 > 3`                      |                   `true`                   |
| **`<=`** | Nhỏ hơn hoặc bằng | `5 <= 5`                     |                   `true`                   |
| **`>=`** | Lớn hơn hoặc bằng | `5 >= 6`                     |                  `false`                   |

### Các lưu ý đặc biệt khi so sánh trong Rust:

#### 1. So sánh chuỗi (`String` và `&str`)

Trong Java, dùng `s1 == s2` sẽ so sánh địa chỉ tham chiếu (phải dùng `.equals()`). Trong Rust, toán tử `==` **so sánh trực tiếp nội dung UTF-8** của 2 chuỗi:

```rust
let s1 = String::from("hello");
let s2 = "hello";
println!("{}", s1 == s2); //  In ra `true`! Cực kỳ tự nhiên!
```

#### 2. Hai vế phải cùng kiểu dữ liệu

```rust
let a: i32 = 5;
let b: u32 = 5;
// println!("{}", a == b); //  LỖI COMPILE: can't compare `i32` with `u32`
println!("{}", a == b as i32); //  Hợp lệ
```

#### 3. Bẫy số thực `NaN` (IEEE 754)

`NaN` (Not a Number) **không bằng bất cứ thứ gì, kể cả chính nó**:

```rust
let nan = f64::NAN;
println!("{}", nan == nan); // in ra `false`!
println!("{}", nan.is_nan()); //  Chuẩn: dùng method `.is_nan()` để kiểm tra
```

Vì lý do này, trong hệ thống trait của Rust, số thực chỉ implement `PartialEq` và `PartialOrd` chứ **không có `Eq` và `Ord`** (không thể sắp xếp mảng float bằng `.sort()` thông thường mà phải dùng `.sort_by()`!).

---

## 5. Toán tử logic & Phủ định (Logical Operators)

Chỉ áp dụng trên kiểu dữ liệu **`bool`** (`true` / `false`).

|  Toán tử   | Tên gọi   | Đặc tính                                                                      | Ví dụ                           |
| :--------: | :-------- | :---------------------------------------------------------------------------- | :------------------------------ |
|  **`&&`**  | AND logic | **Ngắn mạch (short-circuit):** nếu vế trái `false`, vế phải sẽ **không chạy** | `is_admin && has_permission()`  |
| **`\|\|`** | OR logic  | **Ngắn mạch (short-circuit):** nếu vế trái `true`, vế phải sẽ **không chạy**  | `is_owner \|\| is_super_user()` |
|  **`!`**   | NOT logic | Đảo ngược giá trị boolean                                                     | `!is_empty`                     |

### Khác biệt lớn với JS/TS: Không có truthy / falsy

Trong JS bạn có thể viết:

```javascript
// JS: 0, "", null, undefined đều là falsy
if (userName) { ... }
if (items.length) { ... }
```

Trong **Rust, điều kiện của `if` và `while` BẮT BUỘC phải là biểu thức kiểu `bool`**:

```rust
let count = 0;
// if count { }             //  LỖI COMPILE: expected `bool`, found integer
if count > 0 { }            //  Hợp lệ

let name = String::new();
// if name { }              //  LỖI COMPILE: expected `bool`, found `String`
if !name.is_empty() { }     //  Hợp lệ
```

### Cách thay thế toán tử 3 ngôi `? :`

Vì mọi khối `if/else` trong Rust đều là **expression** (có thể trả về giá trị), bạn viết thay thế như sau:

```rust
let score = 85;

// JS / Java: const status = score >= 50 ? "Đỗ" : "Trượt";
// Rust:
let status = if score >= 50 { "Đỗ" } else { "Trượt" };
```

---

## 6. Toán tử thao tác bit (Bitwise Operators)

Thao tác trực tiếp trên từng bit nhị phân của các số nguyên (`u8`, `u32`, `i32`...):

| Toán tử  | Tên gọi                 | Mô tả                                               | Ví dụ nhị phân     |        Kết quả        |
| :------: | :---------------------- | :-------------------------------------------------- | :----------------- | :-------------------: |
| **`&`**  | Bitwise AND             | Cả 2 bit là 1 thì ra 1                              | `0b1100 & 0b1010`  |     `0b1000` (8)      |
| **`\|`** | Bitwise OR              | Có ít nhất 1 bit là 1 thì ra 1                      | `0b1100 \| 0b1010` |     `0b1110` (14)     |
| **`^`**  | Bitwise XOR             | 2 bit khác nhau thì ra 1                            | `0b1100 ^ 0b1010`  |     `0b0110` (6)      |
| **`!`**  | Bitwise NOT             | Đảo toàn bộ các bit (0 thành 1, 1 thành 0)          | `!0b0000_1111u8`   | `0b1111_0000u8` (240) |
| **`<<`** | Dịch trái (Shift left)  | Đẩy bit sang trái, thêm 0 vào đuôi (nhân với $2^n$) | `1u8 << 3`         |          `8`          |
| **`>>`** | Dịch phải (Shift right) | Đẩy bit sang phải (chia cho $2^n$)                  | `16u8 >> 2`        |          `4`          |

> **Lưu ý đặc biệt:**
>
> 1. Trong C/Java/JS, phép đảo bit dùng dấu `~` (`~x`). Trong Rust, **dùng dấu `!`** (`!x`).
> 2. Ký tự `^` là **XOR**, không phải lũy thừa.

---

## 7. Toán tử dải giá trị (Range Operators)

Rust cung cấp cú pháp cực mạnh để tạo các dải giá trị (dùng trong vòng lặp `for`, cắt lát mảng / slice, hoặc pattern matching):

| Cú pháp           | Tên gọi                | Ý nghĩa toán học                      | Ví dụ thực tế                                  |
| :---------------- | :--------------------- | :------------------------------------ | :--------------------------------------------- |
| **`start..end`**  | Nửa mở (Exclusive)     | $start \le x < end$ (không lấy `end`) | `for i in 1..5` (chạy 1, 2, 3, 4)              |
| **`start..=end`** | Đóng (Inclusive)       | $start \le x \le end$ (lấy cả `end`)  | `for i in 1..=5` (chạy 1, 2, 3, 4, 5)          |
| **`start..`**     | Từ `start` đến vô cùng | $x \ge start$                         | `&arr[2..]` (lấy từ phần tử thứ 2 đến hết)     |
| **`..end`**       | Từ đầu đến trước `end` | $x < end$                             | `&arr[..3]` (lấy 3 phần tử đầu: index 0, 1, 2) |
| **`..=end`**      | Từ đầu đến hết `end`   | $x \le end$                           | `&arr[..=3]` (lấy index 0, 1, 2, 3)            |
| **`..`**          | Toàn bộ dải            | Mọi phần tử                           | `&arr[..]` (mượn toàn bộ mảng thành slice)     |

### Ứng dụng trong Pattern Matching:

```rust
let age: u8 = 16;
let category = match age {
    0..=12 => "Trẻ em",
    13..=19 => "Thiếu niên",
    20..=59 => "Người trưởng thành",
    _ => "Người cao tuổi",
};
```

---

## 8. Toán tử tham chiếu & Giải tham chiếu (Borrow & Dereference)

Đây là những toán tử làm nên bản sắc về quyền sở hữu (Ownership) và an toàn bộ nhớ của Rust:

|  Toán tử   | Tên gọi                               | Ý nghĩa                                                                              | Ví dụ             |
| :--------: | :------------------------------------ | :----------------------------------------------------------------------------------- | :---------------- |
|  **`&`**   | Mượn bất biến (Shared Borrow)         | Tạo một tham chiếu chỉ đọc tới giá trị, không chuyển giao quyền sở hữu               | `let r = &x;`     |
| **`&mut`** | Mượn có thể thay đổi (Mutable Borrow) | Tạo một tham chiếu độc quyền cho phép sửa giá trị gốc                                | `let r = &mut x;` |
|  **`*`**   | Giải tham chiếu (Dereference)         | Truy cập trực tiếp vào giá trị nằm tại vùng nhớ mà con trỏ / tham chiếu đang trỏ tới | `*r += 1;`        |

### Ví dụ minh họa `&` và `*`:

```rust
let mut number = 42;
let ref_num = &mut number; // ref_num là tham chiếu &mut i32 trỏ tới number

*ref_num += 10;            // Dùng * để đọc/ghi giá trị vùng nhớ gốc
println!("{number}");      // In ra 52
```

### Cơ chế tự động giải tham chiếu (Deref Coercion):

Khi bạn gọi method qua dấu chấm `.`, Rust sẽ **tự động thêm `&`, `&mut`, hoặc `*`** để khớp với định nghĩa của method đó. Bạn không cần phải viết `(*user).name` như trong C (`user->name`):

```rust
let s = String::from("hello");
let len = s.len(); // Rust tự hiểu và mượn &s khi gọi method .len(&self)
```

---

## 9. Toán tử xử lý lỗi, Pattern Matching & Luồng

### 9.1. Toán tử `?` (Error Propagation / Unwrap or Return)

Toán tử quyền lực nhất khi xử lý lỗi trong Rust:

- Nếu là `Ok(val)` hoặc `Some(val)`: lấy giá trị `val` ra và chạy tiếp.
- Nếu là `Err(e)` hoặc `None`: **tự động `return` lỗi đó ra khỏi hàm hiện tại ngay lập tức**.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // Nếu lỗi, return Err ngay tại đây!
    let mut s = String::new();
    file.read_to_string(&mut s)?;               // Nếu lỗi, return Err!
    Ok(s)                                       // Thành công: bọc trong Ok
}
```

### 9.2. Ký tự gạch dưới `_` (Wildcard Pattern / Ignore)

- Bỏ qua biến không dùng (để compiler không cảnh báo): `let _x = 5;` hoặc `let _ = calculate();`.
- Nhánh mặc định trong `match`:
  ```rust
  match number {
      1 => println!("Một"),
      2 => println!("Hai"),
      _ => println!("Số khác"), // Bắt mọi trường hợp còn lại (giống default trong switch)
  }
  ```

### 9.3. Toán tử `@` (Subpattern Binding)

Vừa kiểm tra điều kiện của pattern, vừa gán giá trị đó vào một biến:

```rust
let msg_id = 5;

match msg_id {
    id @ 1..=10 => println!("ID nhóm ưu tiên thấp: {id}"),
    id @ 11..=100 => println!("ID nhóm trung bình: {id}"),
    _ => println!("ID khác"),
}
```

---

## 10. Toán tử ép kiểu `as`

Dùng để ép kiểu giữa các **kiểu dữ liệu nguyên thủy** (Primitive types):

```rust
let a: i32 = 100;
let b: f64 = a as f64;      // i32 -> f64 (an toàn)
let c: u8 = 65;
let ch: char = c as char;   // u8 -> char ('A')
```

### Cảnh báo nguy hiểm khi dùng `as`:

`as` thực hiện ép kiểu thô (_truncating / sign extension_). Nếu ép từ kiểu số lớn sang số nhỏ mà bị tràn, nó sẽ **âm thầm cắt bỏ các bit cao mà KHÔNG panic**:

```rust
let big: i32 = 300;
let small = big as u8; // 300 % 256 = 44 (âm thầm mất dữ liệu!)
```

> **Khuyên dùng:** Nếu muốn ép kiểu an toàn:
>
> - Dùng `.into()` khi chuyển từ kiểu nhỏ sang kiểu lớn chắc chắn vừa (`i32` -> `i64`).
> - Dùng `TryFrom` / `TryInto` khi chuyển kiểu có nguy cơ tràn (trả về `Result`).

---

## 11. Thứ tự ưu tiên của các toán tử (Operator Precedence)

Xếp từ **ưu tiên cao nhất** (chạy trước) đến **ưu tiên thấp nhất** (chạy sau):

|   Mức độ ưu tiên   | Nhóm toán tử                          | Toán tử cụ thể                                    |
| :----------------: | :------------------------------------ | :------------------------------------------------ |
|  **1 (Cao nhất)**  | Đường dẫn, Gọi hàm, Index, Field, Try | `::`, `()`, `[]`, `.`, `?`                        |
|       **2**        | Toán tử một ngôi (Unary)              | `!`, `-` (đổi dấu), `*` (deref), `&`, `&mut`      |
|       **3**        | Ép kiểu                               | `as`                                              |
|       **4**        | Nhân, Chia, Chia dư                   | `*`, `/`, `%`                                     |
|       **5**        | Cộng, Trừ                             | `+`, `-`                                          |
|       **6**        | Dịch bit                              | `<<`, `>>`                                        |
|       **7**        | Thao tác bit AND                      | `&`                                               |
|       **8**        | Thao tác bit XOR                      | `^`                                               |
|       **9**        | Thao tác bit OR                       | `\|`                                              |
|       **10**       | So sánh thứ tự & Bằng nhau            | `==`, `!=`, `<`, `>`, `<=`, `>=`                  |
|       **11**       | Logic AND                             | `&&`                                              |
|       **12**       | Logic OR                              | `\|\|`                                            |
|       **13**       | Dải giá trị                           | `..`, `..=`                                       |
| **14 (Thấp nhất)** | Phép gán & Gán kết hợp                | `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `\|=`... |

> **Mẹo thực tế:** Thay vì phải căng não ghi nhớ xem giữa `&`, `^`, `|` và `+` cái nào chạy trước, **hãy luôn dùng dấu ngoặc tròn `(...)`** khi viết các biểu thức tính toán phức tạp. Code sẽ vừa rõ ràng vừa an toàn tuyệt đối.

---

## 12. Nâng cao: Operator Overloading với `std::ops`

Trong Rust, các toán tử không phải là ma thuật của compiler mà được liên kết trực tiếp với các **Trait chuẩn** trong module `std::ops`. Bạn có thể nạp chồng (overload) toán tử cho `struct` của riêng mình bằng cách implement trait tương ứng:

|    Toán tử    | Trait tương ứng (`std::ops`) | Method                                   |
| :-----------: | :--------------------------- | :--------------------------------------- |
|      `+`      | `Add`                        | `fn add(self, rhs: Rhs) -> Output`       |
|      `-`      | `Sub`                        | `fn sub(self, rhs: Rhs) -> Output`       |
|      `*`      | `Mul`                        | `fn mul(self, rhs: Rhs) -> Output`       |
|      `/`      | `Div`                        | `fn div(self, rhs: Rhs) -> Output`       |
|      `%`      | `Rem`                        | `fn rem(self, rhs: Rhs) -> Output`       |
|      `!`      | `Not`                        | `fn not(self) -> Output`                 |
| `-` (đổi dấu) | `Neg`                        | `fn neg(self) -> Output`                 |
|     `+=`      | `AddAssign`                  | `fn add_assign(&mut self, rhs: Rhs)`     |
|     `[]`      | `Index`, `IndexMut`          | `fn index(&self, index: Idx) -> &Output` |
|     `==`      | `PartialEq` (`std::cmp`)     | `fn eq(&self, other: &Self) -> bool`     |
|      `<`      | `PartialOrd` (`std::cmp`)    | `fn partial_cmp(...)`                    |

### Ví dụ cộng 2 tọa độ `Point` bằng dấu `+`:

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2; // Gọi trực tiếp toán tử +
    println!("{:?}", p3); // Point { x: 4, y: 6 }
}
```

---

## Tóm tắt nhanh bỏ túi

1. **Không có `++`, `--`** $\rightarrow$ Dùng `+= 1`, `-= 1`.
2. **Không ép kiểu ngầm** $\rightarrow$ Phải cùng kiểu mới tính toán hoặc so sánh được.
3. **Phép gán `=` trả về `()`** $\rightarrow$ Không thể gán bên trong điều kiện `if`.
4. **Không có truthy / falsy** $\rightarrow$ Điều kiện `if` bắt buộc phải là `bool`.
5. **Không có toán tử 3 ngôi `? :`** $\rightarrow$ Dùng `let x = if cond { a } else { b };`.
6. **Không có `null` ngầm hay `?.`** $\rightarrow$ Dùng `Option<T>` và toán tử `?`.
7. **Đảo bit dùng `!`, không dùng `~`**. Ký tự `^` là **XOR**, không phải lũy thừa.
