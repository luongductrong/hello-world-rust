# Rust qua lăng kính JS/TS: Tưởng giống... nhưng "Cua Gắt"

> Tài liệu đối chiếu dành cho người đã có nền **JavaScript / TypeScript** vững và đang chuyển sang học **Rust**.
>
> Rust vay mượn rất nhiều ý tưởng hiện đại (functional, type inference, pattern matching, closure, async/await...) nên nhìn bề ngoài có nhiều chỗ **quen mắt đến bất ngờ**. Nhưng dưới lớp vỏ đó, Rust lại vận hành theo luật chơi **không Garbage Collector (GC) & an toàn bộ nhớ lúc biên dịch**.
>
> Mỗi mục dưới đây đi đúng theo công thức: **Giống JS/TS ở chỗ nào → "But..." trong Rust phải thế này, thế nọ.**
>
> *(Xem thêm phần chi tiết về kiểu số, tràn số và ép kiểu trong [data-types.md](./data-types.md), và ghi chú lộ trình thực hành trong [note.md](./note.md)).*

---

## Mục lục

1. [Truyền hàm làm tham số: Callback và Closure](#1-truyền-hàm-làm-tham-số-callback-và-closure)
2. [Chuỗi xử lý mảng: `.map()`, `.filter()`, `.find()`](#2-chuỗi-xử-lý-mảng-map-filter-find)
3. [Biến bất biến: `let` (Rust) vs `const` (JS)](#3-biến-bất-biến-let-trong-rust-và-const-trong-js)
4. [Destructuring, Shorthand & Struct Update (`..` vs `...`)](#4-destructuring-shorthand-và-struct-update)
5. [Discriminated Union (TS) vs `enum` & `match`](#5-discriminated-union-và-enum-với-match)
6. [`null`/`undefined`, `?.`, `??` vs `Option<T>`](#6-giá-trị-vắng-mặt-null-undefined-và-option)
7. [`interface` & `class` (TS) vs `trait` & `struct/impl`](#7-hợp-đồng-hành-vi-interface-class-và-trait-struct)
8. [`try / catch` & `throw` vs `Result<T, E>` & toán tử `?`](#8-xử-lý-lỗi-try-catch-và-result)
9. [Toán tử 3 ngôi (`? :`), dấu `?` & Tư duy Expression](#9-toán-tử-3-ngôi-dấu-hỏi-và-tư-duy-expression)
10. [Chuỗi ký tự: `string` (JS) vs `String` & `&str` (Rust)](#10-chuỗi-ký-tự-string-và-string-str)
11. [Module & Quản lý gói: `import`/`npm` vs `mod`/`use`/`Cargo`](#11-module-và-quản-lý-gói-npm-và-cargo)
12. [Lập trình bất đồng bộ: `async`/`await` & Đa luồng](#12-lập-trình-bất-đồng-bộ-async-await-và-đa-luồng)
13. [Cheat Sheet tra nhanh: JS/TS sang Rust](#13-cheat-sheet-tra-nhanh-js-và-rust)

---

## 1. Truyền hàm làm tham số: Callback và Closure

### Giống JS/TS
Rust hỗ trợ **First-class functions** và **Closures** (hàm ẩn danh, tương đương arrow function `=>` trong JS):
- Cú pháp gọn nhẹ, tự suy luận kiểu, và **capture (bắt)** được biến từ scope bên ngoài vào dùng.
- Có thể truyền trực tiếp tên một hàm có sẵn (`fn`) hoặc viết inline closure vào làm callback cho hàm khác:

```ts
// TypeScript
const double = (x: number) => x * 2;
const nums = [1, 2, 3];
const found = nums.find((x) => x > 1);
const doubled = nums.map(double);
```

```rust
// Rust
fn double(x: i32) -> i32 {
    x * 2
}

let nums = [1, 2, 3];
let found = nums.iter().find(|&&x| x > 1); // Dùng |x| thay cho (x) =>
let doubled: Vec<i32> = nums.iter().copied().map(double).collect();
```

---

### BUT... 

Trong JS/TS, mọi hàm chỉ có một khái niệm kiểu chung chung kiểu `(x: number) => number`. Trong Rust thì **KHÔNG** — vì không có GC giữ hộ biến ngoài, compiler phải biết chính xác callback **chạm vào biến ngoài theo cách nào**.

#### Điểm khác 1: Không có kiểu chung cho mọi closure — phải chọn `fn(...)` hoặc 1 trong 3 trait `Fn` / `FnMut` / `FnOnce`

Mỗi closure bạn viết trong Rust thực chất được compiler tạo thành một **kiểu dữ liệu ẩn danh (anonymous struct) riêng biệt** chứa các biến nó đã capture. Vì thế, khi viết hàm nhận callback làm tham số, bạn không thể ghi `cb: |i32| -> i32`, mà phải chọn một trong 2 lối sau:

1. **Function Pointer (`fn(i32) -> i32` — chữ `fn` viết thường):**
   - Dùng được cho hàm thường (`fn double(x: i32) -> i32`) và **closure KHÔNG capture bất kỳ biến nào ở ngoài**.
   - **Cấm tuyệt đối** closure có dính tới biến ngoài:
     ```rust
     fn apply(x: i32, f: fn(i32) -> i32) -> i32 {
         f(x)
     }

     let k = 10;
     apply(5, |x| x + 1); // OK: không capture biến ngoài
     // apply(5, |x| x + k); // LỖI COMPILE! Có capture `k`, không ép sang `fn` được
     ```

2. **3 Trait của Closure (`Fn`, `FnMut`, `FnOnce`) — dùng qua `impl ...` hoặc Generic `<F: ...>`:**
   Đây mới là thứ nhận được mọi loại callback (cả hàm thường lẫn closure có capture biến ngoài), chia thành **3 cấp độ quyền lực**:

   | Trait | Callback làm gì với biến ngoài? | Gọi được mấy lần? | Ví dụ dùng |
   | :--- | :--- | :--- | :--- |
   | **`Fn`** | Chỉ **đọc (`&`)** hoặc không capture gì | Vô số lần (kể cả chia sẻ song song) | Validate, transform thuần túy |
   | **`FnMut`** | **Sửa (`&mut`)** biến bên ngoài | Nhiều lần tuần tự (biến chứa cb phải `mut`) | `.map()`, `.filter()`, `.for_each()` (đếm biến `count += 1`) |
   | **`FnOnce`** | Có thể **Move (nuốt mất)** biến ngoài vào trong và tiêu thụ nó | **Tối đa 1 lần** | `.unwrap_or_else(|| ...)` (cùng lắm chạy 1 lần) |

   ```rust
   // 1. Chỉ gọi 1 lần -> ưu tiên FnOnce (rộng rãi nhất cho người gọi)
   fn run_once(cb: impl FnOnce()) {
       cb();
   }

   // 2. Gọi nhiều lần và cho phép callback thay đổi biến bên ngoài -> FnMut
   fn run_three_times(mut cb: impl FnMut(i32)) {
       for i in 1..=3 {
           cb(i);
       }
   }

   // 3. Callback chỉ đọc (không được sửa biến ngoài) -> Fn
   fn transform_all(nums: &[i32], cb: impl Fn(i32) -> i32) -> Vec<i32> {
       nums.iter().map(|&x| cb(x)).collect()
   }
   ```

   **Quy tắc kế thừa giữa 3 trait:** `Fn` ⊂ `FnMut` ⊂ `FnOnce`.
   - Closure kiểu `Fn` dùng được ở mọi chỗ đòi `Fn`, `FnMut`, hay `FnOnce`.
   - Chỗ đòi `Fn` sẽ **từ chối** closure có `count += 1` (vì nó cần `FnMut`) hoặc `drop(name)` (vì nó cần `FnOnce`).

#### Điểm khác 2: Capture theo Ownership/Borrowing — dùng `move` khi trả closure ra ngoài

Trong JS, hàm trả về một hàm (factory / currying) cực kỳ tự nhiên vì GC giữ biến `base` sống mãi:
```js
const makeAdder = (base) => (x) => base + x;
```

Trong Rust, compiler mặc định chỉ cho closure **mượn (`&`)** biến `base`. Nếu bạn trả closure ra khỏi hàm `make_adder`, biến `base` sẽ bị hủy (`drop`) ngay khi `make_adder` kết thúc → tham chiếu bị treo! Vì vậy bạn bắt buộc phải:
1. Gắn **`move`** trước closure để chuyển hẳn quyền sở hữu (hoặc copy giá trị) vào trong closure.
2. Ghi kiểu trả về là **`impl Fn(...) -> ...`** (hoặc `Box<dyn Fn(...)>` nếu trả về nhiều loại closure khác nhau ở các nhánh `if/else`).

```rust
fn make_adder(base: i32) -> impl Fn(i32) -> i32 {
    move |x| base + x // `move` nhét `base` vào trong closure để sống tiếp sau khi hàm return
}
```

#### Điểm khác 3: Không thể bỏ nhiều closure khác nhau vào cùng 1 `Vec` như JS

- **JS:** `const handlers = [() => log("A"), () => count++];` — quá đơn giản.
- **Rust:** Vì mỗi closure có một kiểu ẩn danh riêng khác kích thước nhau, `Vec` không thể chứa trực tiếp 2 kiểu khác nhau. Muốn lưu mảng callback (như event listener), bạn phải bọc qua **Trait Object** trên Heap:
  ```rust
  let handlers: Vec<Box<dyn Fn()>> = vec![
      Box::new(|| println!("Handler 1")),
      Box::new(|| println!("Handler 2")),
  ];
  ```

#### Điểm khác 4: Closure tự suy luận kiểu, nhưng bị "khóa cứng" ngay lần gọi đầu

```ts
// TS: hàm generic đa hình
const identity = <T>(x: T): T => x;
identity(1);
identity("hello"); // OK
```

```rust
// Rust: nhìn thì giống nhưng KHÔNG phải generic
let identity = |x| x;
let a = identity(1);
// let b = identity("hello"); // LỖI COMPILE! Kiểu của `x` đã bị chốt chết là số nguyên từ dòng trên
```

---

## 2. Chuỗi xử lý mảng: map, filter, find

### Giống JS/TS
Rust có đầy đủ bộ method phong cách functional trên Iterator: `.map()`, `.filter()`, `.find()`, `.any()`, `.all()`, `.fold()` (giống `.reduce()`), `.enumerate()` (lấy kèm index như `(item, i)`).

### BUT...

1. **JS chạy ngay (Eager) & tốn mảng trung gian vs Rust "lười" (Lazy) & 0 cấp phát trung gian:**
   - **JS:** `arr.filter(...).map(...)` chạy vòng lặp ngay ở `.filter()` tạo ra **1 mảng mới trên Heap**, rồi chạy tiếp `.map()` tạo thêm **1 mảng mới nữa**.
   - **Rust:** `vec.iter().filter(...).map(...)` **chưa chạy gì cả**! Nó chỉ lắp ráp đường ống xử lý (*lazy iterator*). Chỉ khi bạn gọi điểm cuối (*consumer*) như `.collect::<Vec<_>>()`, `.sum()`, `.count()`, hoặc lặp bằng `for`, Rust mới chạy **duy nhất 1 vòng lặp** cho toàn bộ chuỗi (*zero-cost abstraction*).

2. **Phải chọn rõ 1 trong 3 cách mở iterator:**
   - JS chỉ có một kiểu duyệt qua tham chiếu phần tử.
   - Rust ép chọn rõ mục đích ngay từ đầu:
     - `.iter()` → duyệt từng `&T` (chỉ đọc, giữ nguyên mảng/Vec gốc).
     - `.iter_mut()` → duyệt từng `&mut T` (sửa trực tiếp từng phần tử).
     - `.into_iter()` → **Move** từng `T` ra ngoài (tiêu thụ sạch, dùng xong `Vec` gốc biến mất).

3. **Cú pháp `&&x` gây lú ở `.filter()`:**
   Vì `nums.iter()` đã nhả ra `&i32`, mà `.filter()` lại cho callback mượn thêm một lớp `&Item` nữa nên tham số của `.filter()` thành `&&i32`!
   - Cách xử lý gọn cho kiểu `Copy` (số, `bool`, `char`): dùng `.copied()` trước, hoặc destructure `&` ngay trong tham số:
     ```rust
     let nums = [1, 2, 3, 4];
     // Cách 1: dùng .copied() cho dễ thở như JS
     let evens: Vec<i32> = nums.iter().copied().filter(|x| x % 2 == 0).map(|x| x * 2).collect();

     // Cách 2: bóc & trực tiếp
     let evens2: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).collect();
     ```

---

## 3. Biến bất biến: let trong Rust và const trong JS

### Giống JS/TS
Cả hai đều có block scope `{ ... }` và khuyến khích mặc định dùng biến không gán lại được (`const` ở JS ↔ `let` ở Rust).

### BUT...

1. **Bất biến nông (JS `const`) vs Bất biến sâu (Rust `let`):**
   - **JS:** `const` chỉ khóa cái **tên biến** không trỏ sang object khác, còn ruột bên trong sửa tẹt ga:
     ```js
     const user = { name: "An" };
     user.name = "Bình"; // Hợp lệ!
     const arr = [1, 2];
     arr.push(3);        // Hợp lệ!
     ```
   - **Rust:** `let user = ...;` khóa **toàn bộ dữ liệu** mà `user` sở hữu. Không thể sửa field, không thể `.push()` vào `Vec` nếu biến đó không có `mut` (`let mut`).

2. **`const` trong Rust KHÔNG phải `const` của JS:**
   - **JS:** `const data = await fetchUser();` — tính lúc chạy (runtime) bình thường.
   - **Rust:** `let` mới tương đương `const` dùng hàng ngày của JS. Còn `const` của Rust là **hằng số lúc biên dịch** (*compile-time constant*), bắt buộc viết HOA, bắt buộc ghi rõ kiểu và không được chứa giá trị tính lúc runtime:
     ```rust
     const MAX_RETRY: u32 = 3;
     ```

3. **Shadowing (Khai báo đè cùng tên):**
   - **JS/TS:** Khai báo lại `const x` hoặc `let x` trong cùng scope là **SyntaxError**.
   - **Rust:** `let x = "42"; let x: i32 = x.parse().unwrap();` là **hoàn toàn hợp lệ** và rất được khuyến khích để tái sử dụng tên biến khi biến đổi kiểu dữ liệu (tránh phải đặt `rawInput`, `trimmedInput`, `parsedInput`...).

---

## 4. Destructuring, Shorthand và Struct Update

### Giống JS/TS
Cú pháp bóc tách và viết tắt struct/object gần như anh em sinh đôi:
- Field shorthand: `User { id, name }` ↔ `{ id, name }`.
- Destructuring: `let User { id, name } = user;` ↔ `const { id, name } = user;`.
- Update phần còn lại: `User { active: false, ..u1 }` ↔ `{ ...u1, active: false }`.

### BUT...

1. **`..u1` có thể làm "xẻ thịt" biến cũ (Partial Move):**
   - **JS:** `{ ...u1, active: false }` tạo object mới, các thuộc tính dạng object/string được copy tham chiếu nông, `u1` cũ vẫn dùng nguyên vẹn.
   - **Rust:** Các field không phải `Copy` (như `String`, `Vec`) bên trong `u1` sẽ bị **Move** sang `u2`! Sau dòng đó, `u1` rơi vào trạng thái *partially moved*: field `Copy` như `u1.id` vẫn đọc được, nhưng `u1.name` và cả cục `u1` không dùng được nữa (trừ khi tự `.clone()` tay từng field).
   ```rust
   let u1 = User { id: 1, name: String::from("An"), active: true };
   let u2 = User { active: false, ..u1 };
   // println!("{}", u1.name); // LỖI COMPILE: `u1.name` đã bị Move sang `u2`!
   println!("{}", u1.id);      // OK vì `u32` là `Copy`
   ```

2. **Thứ tự đặt `..` ngược hẳn `...` của JS:**
   - **JS:** Muốn ghi đè `active`, phải đặt `...u1` **trước**: `{ ...u1, active: false }`. Nếu đặt sau (`{ active: false, ...u1 }`), `u1.active` sẽ đè ngược lại `false`!
   - **Rust:** `..u1` **bắt buộc luôn đứng cuối cùng**, và chỉ có nghĩa *"điền nốt những field chưa được liệt kê ở trên"*.

3. **Tương tự khi Destructuring:**
   `let User { name, .. } = u1;` sẽ **Move** `name` ra khỏi `u1`. Nếu chỉ muốn đọc mà không làm hỏng `u1`, phải destructure qua tham chiếu: `let User { name, .. } = &u1;` (lúc này `name` có kiểu `&String`). Ngoài ra Rust không có cú pháp `...rest` để gom các field còn lại thành một struct mới.

---

## 5. Discriminated Union và enum với match

### Giống JS/TS
Nếu bạn từng thích **Discriminated Union** trong TypeScript thì `enum` của Rust chính là phiên bản nâng cấp tối thượng:

```ts
// TypeScript
type Shape =
  | { kind: "circle"; radius: number }
  | { kind: "rect"; w: number; h: number };
```

```rust
// Rust
enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
}
```

Cả hai đều có kiểm tra vét cạn (*exhaustiveness check*) — quên xử lý một trường hợp là báo lỗi ngay lúc biên dịch.

### BUT...

1. **Tag tích hợp sẵn, không phải "giả lập" bằng field chuỗi:**
   Trong TS bạn phải tự chế field `kind: "circle"` và vẫn có thể lỡ ép kiểu `as any` ở runtime. Trong Rust, `enum` là kiểu tổng (Sum Type) chuẩn của ngôn ngữ, gắn trực tiếp vào bộ nhớ và thậm chí còn viết được `impl Shape { ... }` như `struct`.

2. **`match` ăn đứt `switch`:**
   - Không có *fall-through* (không cần `break;`).
   - Vừa rẽ nhánh vừa destructure dữ liệu bên trong ngay tại chỗ, hỗ trợ gộp nhánh (`1 | 2`), khoảng (`1..=5`), điều kiện kèm (`if x > 0`).
   - Là một **expression** trả về giá trị trực tiếp.

3. **Cẩn thận bẫy Move khi `match`:**
   `match shape { ... }` sẽ **Move** dữ liệu bên trong `shape` ra ngoài! Nếu chỉ muốn xem rồi giữ lại `shape` để dùng tiếp, phải viết `match &shape` (hoặc `match &mut shape` nếu muốn sửa tại chỗ).

---

## 6. Giá trị vắng mặt: null, undefined và Option

### Giống JS/TS
Cả hai đều có cơ chế xử lý an toàn cho giá trị có thể vắng mặt:
- TS dùng `T | undefined`, optional chaining `?.`, và nullish coalescing `??`.
- Rust dùng `Option<T>` (`Some(v)` hoặc `None`), `.map()` / `.and_then()`, và `.unwrap_or()` / `.unwrap_or_else()`.

### BUT...

1. **Không có `null`, không có `undefined`, và KHÔNG có "truthy / falsy":**
   - **JS:** `if (count)` coi `0`, `""`, `NaN`, `null`, `undefined`, `false` đều là falsy → cực dễ dính bug logic khi `0` hoặc `""` là giá trị hợp lệ!
   - **Rust:** `if` **chỉ nhận `bool`**. `Option<i32>` và `i32` là **hai kiểu khác hẳn nhau** → compiler ép bạn mở hộp `Some`/`None` trước khi chạm vào dữ liệu bên trong. Không bao giờ có `Cannot read properties of undefined`.

2. **Bẫy hiệu năng ngầm giữa `??` (JS) và `.unwrap_or()` (Rust):**
   Đây là bẫy dev JS rất hay vấp phải!
   - **JS:** `const port = envPort ?? getDefaultPort();` → `getDefaultPort()` **chỉ chạy khi `envPort` là nullish** (short-circuit / lazy).
   - **Rust:**
     ```rust
     // CẨN THẬN: get_default_port() LUÔN CHẠY TRƯỚC, kể cả khi env_port là Some(3000)!
     let port = env_port.unwrap_or(get_default_port());

     // ĐÚNG (giống ?? của JS): truyền closure (hoặc tên hàm trực tiếp) để chạy lazy!
     let port = env_port.unwrap_or_else(|| get_default_port());
     let port = env_port.unwrap_or_else(get_default_port); // Gọn hơn nếu không cần truyền tham số
     ```
   *Quy tắc nhớ nhanh:* Fallback là hằng số rẻ tiền (`0`, `false`) → dùng `.unwrap_or(0)`. Fallback là hàm tính toán hoặc cấp phát chuỗi `String::from(...)` → **luôn dùng `.unwrap_or_else(...)`**.

---

## 7. Hợp đồng hành vi: interface, class và trait, struct

### Giống JS/TS
`trait` trong Rust đóng vai trò bản hợp đồng hành vi rất giống `interface` trong TypeScript (mô tả các method cần có, hỗ trợ cả generic constraint `<T: MyTrait>` giống `<T extends MyInterface>`).

### BUT...

1. **Duck Typing (TS) vs Nominal Explicit Implementation (Rust):**
   - **TS (Structural Typing):** Chỉ cần một object có đủ các field/method trùng tên & kiểu với `interface Speaker`, TS tự coi nó là `Speaker`.
   - **Rust:** Kể cả `struct Dog` đã có sẵn hàm `fn speak(&self)`, nó **vẫn chưa implement `trait Speak`** cho đến khi bạn viết tường minh:
     ```rust
     impl Speak for Dog {
         fn speak(&self) -> String {
             String::from("Gâu!")
         }
     }
     ```
   *Điểm cộng cực lớn của `trait`:* Bạn có thể gắn thêm method mới (implement một `trait` của bạn) cho kiểu có sẵn như `i32`, `String`, hoặc struct từ thư viện ngoài (miễn là bạn sở hữu `trait` hoặc kiểu đó — gọi là *Orphan Rule*). JS/TS mà sửa `String.prototype` là bị "ăn gậy" ngay!

2. **Không có `class` và không có `extends` (Kế thừa):**
   - Rust tách hoàn toàn dữ liệu (`struct` / `enum`) khỏi hành vi (`impl`). Không có chuỗi prototype, không có `super()`, không có kế thừa `Admin extends User`.
   - Thay vào đó Rust dùng **Composition + Traits**.

3. **Không có `this` "lật mặt" và từ khóa `new`:**
   - **JS:** `this` phụ thuộc vào *cách gọi hàm* (`const f = obj.method; f()` làm mất `this`, phải `.bind(obj)` hoặc dùng arrow function).
   - **Rust:** Tham số đầu tiên luôn ghi rõ `&self` (đọc), `&mut self` (sửa), hoặc `self` (nuốt mất quyền sở hữu). Không có từ khóa `new`; hàm tạo chỉ là quy ước đặt tên `Self::new(...)` (associated function, giống `static` factory).

4. **Đa hình Tĩnh (`impl Trait`) vs Động (`dyn Trait`):**
   - **TS:** `interface` biến mất lúc biên dịch; mọi object chạy qua cơ chế tham chiếu động ở runtime.
   - **Rust:** Mặc định khi viết `fn say(a: &impl Speak)` (hoặc `<T: Speak>`), compiler tự sinh bản code tối ưu riêng cho từng kiểu cụ thể lúc compile (*Static Dispatch / Monomorphization* — không tốn chi phí tra bảng ảo). Chỉ khi nào bạn cần chứa hỗn hợp nhiều kiểu khác nhau trong cùng một mảng (`Vec<Box<dyn Speak>>`), bạn mới bật *Dynamic Dispatch* với từ khóa `dyn`.

---

## 8. Xử lý lỗi: try catch và Result

### Giống JS/TS
Đều cho phép hàm cấp dưới báo lỗi và đẩy trách nhiệm xử lý lỗi ngược lên cho hàm cấp trên gọi nó.

### BUT...

1. **Lỗi ẩn trong bóng tối (JS) vs Lỗi nằm lộ thiên trên chữ ký hàm (Rust):**
   - **JS/TS:** Ngay cả trong TypeScript, `function parseConfig(s: string): Config` **không hề cho biết** hàm này có thể `throw` những loại lỗi gì! Quên bọc `try/catch` là app crash lúc runtime.
   - **Rust:** Không có `try / catch` và không có `throw`. Lỗi có thể phục hồi được trả về qua `Result<T, E>` (trong đó `T` là kiểu khi thành công, `E` là kiểu lỗi). Nhìn chữ ký hàm là thấy ngay khả năng lỗi, và compiler **ép bạn phải xử lý** mới lấy được `T` ra dùng.

2. **Toán tử `?` thay thế gọn gàng cho việc ném lỗi lên trên:**
   Thay vì phải viết khối `try { ... } catch (e) { throw e; }`, Rust đặt `?` ngay cuối biểu thức có thể lỗi:
   ```rust
   fn read_port(s: &str) -> Result<u16, std::num::ParseIntError> {
       let port = s.trim().parse::<u16>()?; // Nếu Err -> tự return Err sớm ngay tại đây!
       Ok(port)
   }
   ```

3. **`panic!` KHÔNG phải `throw`:**
   Đừng dùng `panic!` (hay `.unwrap()` bừa bãi) để thay cho `throw` trong nghiệp vụ thông thường. `panic!` dành cho **lỗi bất biến / bug nghiêm trọng không thể cứu vãn** (như truy cập ngoài biên mảng), chứ không phải cơ chế flow control để bắt lại như `try/catch`.

---

## 9. Toán tử 3 ngôi, dấu hỏi và tư duy Expression

### Giống JS/TS
Cả hai đều muốn viết gán giá trị theo điều kiện ngắn gọn trên 1 dòng.

### BUT...

1. **Không có toán tử 3 ngôi `cond ? a : b`:**
   Trong Rust, dấu **`?` đã dành riêng để truyền lỗi (`Result`/`Option`)** rồi! Thay cho `? :`, Rust biến chính `if / else` (và `match`, và cả block `{ ... }`) thành **expression trả về giá trị**:
   ```rust
   // JS: const label = score >= 5 ? "Đậu" : "Rớt";
   let label = if score >= 5 { "Đậu" } else { "Rớt" };
   ```
   *(Chú ý: không có `;` sau `"Đậu"` và `"Rớt"`, nhưng có `;` sau dấu `}` cuối cùng của lệnh `let`).*

2. **Dấu `;` làm thay đổi hoàn toàn ý nghĩa dòng cuối trong `{ ... }`:**
   - **JS:** `{ return x + 1; }` — thiếu `return` là hàm trả về `undefined`.
   - **Rust:** Dòng cuối của block/hàm **không có `;`** tự động là giá trị trả về (giống implicit return `=> x + 1` của arrow function JS). Lỡ tay thêm `;` vào cuối, block biến thành trả về `()` (*unit type*, giống `void`). Còn khi muốn thoát sớm giữa hàm (`if ... { return ...; }`), **bắt buộc dùng từ khóa `return`**.

---

## 10. Chuỗi ký tự: string và String, str

### Giống JS/TS
Đều dùng nháy kép `"..."`, có chuỗi template tiện lợi (`format!("Xin chào {name}")` ↔ `` `Xin chào ${name}` ``), và có các method quen thuộc như `.trim()`, `.contains()`, `.starts_with()`, `.split()`.

### BUT...

1. **Một `string` (JS) vs Hai kiểu `String` và `&str` (Rust):**
   - **JS:** `string` là bất biến, được GC quản lý. Cắt chuỗi `.slice()` hay nối chuỗi tạo chuỗi mới tự động.
   - **Rust:**
     - `String`: sở hữu vùng nhớ trên Heap, co giãn/sửa nội dung được (`push_str`, `clear`).
     - `&str` (*string slice*): chỉ là góc nhìn (con trỏ + độ dài) trỏ vào chuỗi có sẵn, chỉ đọc, không tốn cấp phát Heap. Hàm nhận tham số chỉ đọc nên luôn nhận `&str` (vì truyền được cả `"literal"` lẫn `&my_string`).

2. **Cấm truy cập `s[0]` theo index:**
   - **JS:** `"Xin"[0]` cho `"X"`, nhưng với emoji `"🦀"[0]` lại ra nửa ký tự UTF-16 bị hỏng (`'\ud83e'`).
   - **Rust:** Chuỗi luôn mã hóa **UTF-8** (ký tự dài từ 1 đến 4 bytes). Rust **cấm thẳng `s[0]` lúc compile** để bạn không vô tình cắt đôi một ký tự tiếng Việt có dấu hoặc emoji, và tránh hiểu nhầm độ phức tạp `O(1)` thành `O(n)`. Muốn duyệt từng ký tự, dùng `s.chars()`.

---

## 11. Module và quản lý gói: npm và Cargo

### Giống JS/TS
Hệ sinh thái cực kỳ quen tay nếu bạn đã dùng `npm`/`pnpm`:
- [`Cargo.toml`](./Cargo.toml) ↔ `package.json`
- [`Cargo.lock`](./Cargo.lock) ↔ `package-lock.json` / `pnpm-lock.yaml`
- `crates.io` ↔ `npm` registry (`cargo add serde` ↔ `npm install serde`)
- `pub` ↔ `export` (trong Rust mọi thứ mặc định **private**, phải gắn `pub` mới lộ ra ngoài)
- `use` ↔ `import`

### BUT...

1. **`import` tự kéo file (JS) vs `use` KHÔNG tự nạp file (Rust):**
   - **JS/TS:** Chỉ cần gõ `import { foo } from "./bar";` là bundler tự tìm và nạp file `bar.ts`.
   - **Rust:** `use` chỉ là **tạo đường tắt tên gọi** (*alias*), hoàn toàn không nạp file! Một file mới như `src/bar.rs` chỉ tồn tại trong project khi file cha (`src/main.rs` hoặc `src/lib.rs`) khai báo rõ **`mod bar;`**.

2. **Không có `node_modules` phình to từng project & Tooling tất cả trong một:**
   - JS thường ghép nối `npm/pnpm` + `tsc` + `eslint` + `prettier` + `vitest/jest` + `vite`.
   - Rust gom hết vào một đầu mối `cargo`: `cargo check` (check nhanh), `cargo clippy` (linter), `cargo fmt` (formatter), `cargo test` (test runner), `cargo build --release` (build tối ưu). Các thư viện tải về được cache chung toàn máy, không nhân bản hàng trăm MB `node_modules` vào mỗi repo.

---

## 12. Lập trình bất đồng bộ: async await và đa luồng

*(Phần này bạn sẽ gặp ở các chặng nâng cao hơn, nắm trước tư duy để khỏi bỡ ngỡ)*

### Giống JS/TS
Rust cũng dùng `async fn` và `await` để viết code bất đồng bộ nhìn tuần tự, dễ đọc:
```rust
let data = fetch_user(id).await?;
```

### BUT...

1. **Vị trí `.await` nằm ở ĐUÔI:** JS viết `await fetchUser(id)`, Rust viết `fetch_user(id).await` (để nối chuỗi cực mượt với toán tử `?`, ví dụ `fetch_user(id).await?.name`).
2. **JS `Promise` là Eager vs Rust `Future` là Lazy & Không có Runtime tích hợp:**
   - **JS:** Vừa gọi `const p = fetch(url);` là `Promise` đã tự động chạy ngầm trên Event Loop có sẵn của Node.js / Browser rồi.
   - **Rust:** Thư viện chuẩn không gắn sẵn Event Loop. Gọi một `async fn` chỉ trả về một `Future` **đứng im không chạy gì cả**, cho tới khi bạn `.await` nó hoặc ném vào một async runtime bên ngoài (phổ biến nhất là `tokio`).
3. **Đa luồng thật (Multi-threading) không sợ Data Race:**
   - JS mặc định đơn luồng (single-threaded Event Loop), muốn chạy nhiều nhân CPU phải mở Worker gửi message qua lại.
   - Rust chạy **đa luồng OS thật sự** (`std::thread` hoặc multi-thread executor), chia sẻ dữ liệu trực tiếp qua bộ nhớ an toàn nhờ Borrow Checker + 2 trait `Send`/`Sync` chặn lỗi tranh chấp dữ liệu (*Data Race*) **ngay lúc biên dịch**.

---

## 13. Cheat Sheet tra nhanh: JS và Rust

| JS / TS | Rust tương đương | Cú "cua gắt" (Khác biệt quan trọng nhất) |
| :--- | :--- | :--- |
| `const x = 1` | `let x = 1;` | Rust `let` bất biến **sâu** (khóa cả ruột struct/Vec); `const` là hằng số **compile-time** |
| `let x = 1` | `let mut x = 1;` | Khai báo lại `let x = ...` cùng tên (Shadowing) hoàn toàn hợp lệ |
| `(x) => x + 1` | `\|x\| x + 1` | Tham số nhận callback phải phân biệt `fn` vs `Fn` / `FnMut` / `FnOnce`; dùng `move` nếu trả closure ra ngoài |
| `arr.map(f).filter(g)` | `v.iter().map(f).filter(g).collect()` | Rust **Lazy** (chỉ chạy ở `.collect()`/consumer); chọn rõ `.iter()`, `.iter_mut()`, hay `.into_iter()` |
| `{ ...obj, a: 1 }` | `Obj { a: 1, ..obj }` | `..obj` luôn đứng **cuối** và sẽ **Move** các field không phải `Copy` (làm hỏng `obj` cũ) |
| `type U = { tag: ... }` | `enum U { ... }` | Native sum type, bắt buộc `match` hết 100% nhánh; `match &u` nếu không muốn bị Move |
| `x ?? fallback()` | `x.unwrap_or_else(\|\| fallback())` | `.unwrap_or(fallback())` chạy **eager** (luôn gọi `fallback()` trước kể cả khi có giá trị!) |
| `x?.y` | `.map(\|v\| v.y)` hoặc `?` | Không có `null`/`undefined`, không có truthy/falsy; `if` chỉ nhận `bool` |
| `cond ? a : b` | `if cond { a } else { b }` | Dấu `?` dành cho lỗi; block `{ ... }` bỏ `;` dòng cuối để trả về giá trị |
| `interface` + `class` | `trait` + `struct` / `impl` | Không `extends`, không duck typing (phải `impl Trait for Type`), không `this` ngầm |
| `try / catch` + `throw` | `Result<T, E>` + `?` | Lỗi nằm thẳng trên kiểu trả về; `panic!` không phải `throw` để bắt lại |
| `string` | `String` (Heap) & `&str` (slice) | Chuỗi UTF-8, cấm `s[0]`, dùng `s.chars()` |
| `import` + `package.json` | `mod` + `use` + [`Cargo.toml`](./Cargo.toml) | `use` không nạp file; file cha phải khai báo `mod ten_file;` |
| `await fn()` (Promise) | `fn().await` (Future) | Future là **Lazy** và cần runtime ngoài (như `tokio`) để chạy |
