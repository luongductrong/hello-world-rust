# Lộ trình học Rust qua thực hành (Interactive Roadmap)

> Phương pháp: Học qua **trao đổi + viết code chạy thử ngay**, mỗi khái niệm gắn với tình huống thực tế và thông báo lỗi của compiler. Không đọc sách chay từ đầu đến cuối.
> Nền tảng đối chiếu: **JavaScript/TypeScript** (mạnh), **Java** (khá), **C** (cơ bản).

Tham khảo các cẩm nang tra cứu:

- [**Tương quan giữa Rust và JavaScript/TypeScript**](./rust-vs-js-ts.md)
- [**Hệ thống Kiểu dữ liệu & Tràn số**](./data-types.md)
- [**Toán tử, Phép tính & So sánh toàn diện**](./operators.md)

## Chặng 0: Bộ công cụ Cargo & Môi trường _(Hoàn thành)_

- [x] Xử lý lỗi Windows `os error 4551` (bật Developer Mode) để chạy được `cargo run`
- [x] Hiểu cấu trúc project Cargo ([`Cargo.toml`](./Cargo.toml), [`Cargo.lock`](./Cargo.lock), [`src/main.rs`](./src/main.rs), [`.gitignore`](./.gitignore))
- [x] Nắm quy trình lệnh hàng ngày: `cargo check`, `cargo run`, `cargo fmt`, `cargo clippy` _(riêng `cargo test` sẽ thực hành kỹ ở Chặng 4)_
- [x] Cài `rust-analyzer` trong VS Code _(sẽ dùng xuyên suốt qua các bài tập)_

## Chặng 1: Cú pháp cốt lõi & Tư duy biểu thức (Expression-oriented) _(Hoàn thành)_

- [x] Biến bất biến mặc định (`let` vs `let mut`, `const`) _(áp dụng shadowing khi đọc & parse input)_
- [x] Kiểu dữ liệu cơ bản & tràn số (xem chi tiết tại [`data-types.md`](./data-types.md))
- [x] Các toán tử, phép tính & so sánh (xem chi tiết tại [`operators.md`](./operators.md))
- [x] Hàm: statement vs expression (dấu `;` cuối hàm và block `{ }`)
- [x] Điều khiển luồng: `if` trả về giá trị, `loop`, `while`, `for` với range `1..=10` _(áp dụng `break value;` trong bài tập nhỏ)_
- [x] **Bài tập kết Chặng 1:** Mini-game Đoán số CLI (gộp `const`, `u32`, `shadowing`, `tuple`, `loop` trả giá trị, `std::io`, `parse`) ([`src/guessing_game.rs`](./src/guessing_game.rs))

## Chặng 2: Trái tim của Rust — Ownership, Borrowing & Slices _(Hoàn thành)_

- [x] Stack vs Heap qua cặp đôi `String` và `&str`
- [x] Move semantics: vì sao truyền biến vào hàm xong không dùng lại được?
- [x] Reference & Borrowing: `&T` (shared) vs `&mut T` (exclusive)
- [x] Hiểu và "làm bạn" với thông báo lỗi của Borrow Checker
- [x] Slices (`&str`, `&[i32]`) _(đã thực hành `&str` với `first_word`; `&[T]` sẽ áp dụng cùng `Vec<T>` ở Chặng 4)_

## Chặng 3: Dữ liệu & Trạng thái an toàn — Struct, Enum, Option/Result _(Hoàn thành)_

- [x] `struct` + `impl` (methods `&self`, `&mut self`, `self` và `Self::new`)
- [x] `enum` mang dữ liệu kèm theo
- [x] Pattern matching: `match`, `if let`, `let ... else`
- [x] `Option<T>` (cách Rust thay thế `null`)
- [x] `Result<T, E>` (cách Rust thay thế `try / catch` & Exception)
- [x] **Bài tổng hợp kết Chặng 3:** Máy bán nước tự động mini ([`src/vending_machine.rs`](./src/vending_machine.rs))

## Chặng 4: Collections, Error Handling & Tổ chức code _(Hoàn thành)_

- [x] Buổi 4.1: `Vec<T>`, `HashMap<K, V>` & Entry API ([`src/lesson_4_collections.rs`](./src/lesson_4_collections.rs))
- [x] Buổi 4.2: Closures & Iterators (`.iter()`, `.into_iter()`, `.iter_mut()`, `.map()`, `.filter()`, `.collect()`, `.find()`) ([`src/lesson_4_iterators.rs`](./src/lesson_4_iterators.rs))
- [x] Buổi 4.3: Xử lý lỗi nâng cao & Toán tử `?` (thay cho `.unwrap()`) ([`src/lesson_4_errors.rs`](./src/lesson_4_errors.rs))
- [x] Buổi 4.4: Tổ chức module (`mod`, `pub`, `use`) & Unit test với `#[cfg(test)]`, `cargo test` ([`src/lesson_4_errors.rs`](./src/lesson_4_errors.rs))

## Chặng 5: Traits, Generics & Mini-Project tổng hợp _(Đang học)_

- [x] Buổi 5.1: Traits cơ bản & Derive (`Display`, `Debug`, `Clone`, `PartialEq`, Default methods) ([`src/lesson_5_traits.rs`](./src/lesson_5_traits.rs))
- [x] Buổi 5.2: Generics `<T>` & Trait Bounds (`impl Trait`, `where` clause, `T: Display + Clone`) ([`src/lesson_5_generics.rs`](./src/lesson_5_generics.rs))
- [x] Buổi 5.3: Static Dispatch (`impl Trait`) vs Dynamic Dispatch (`Box<dyn Trait>`) ([`src/lesson_5_dispatch.rs`](./src/lesson_5_dispatch.rs))
- [ ] Buổi 5.4: **Mini-Project Capstone Chặng 5:** Ứng dụng CLI Quản lý chi tiêu/công việc (lưu file JSON với `serde`, parse CLI args)
- [ ] Buổi 5.4: **Mini-Project Capstone Chặng 5:** Ứng dụng CLI Quản lý chi tiêu/công việc (lưu file JSON với `serde`, parse CLI args)
