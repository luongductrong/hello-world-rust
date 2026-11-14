# Lộ trình học Rust qua thực hành (Interactive Roadmap)

> Phương pháp: Học qua **trao đổi + viết code chạy thử ngay**, mỗi khái niệm gắn với tình huống thực tế và thông báo lỗi của compiler. Không đọc sách chay từ đầu đến cuối.
> Nền tảng đối chiếu: **JavaScript/TypeScript** (mạnh), **Java** (khá), **C** (cơ bản).

Tham khảo [**Tương quan giữa Rust và JavaScript/TypeScript**](/rust-vs-js-ts.md)

## Chặng 0: Bộ công cụ Cargo & Môi trường *(Hoàn thành)*
- [x] Xử lý lỗi Windows `os error 4551` (bật Developer Mode) để chạy được `cargo run`
- [x] Hiểu cấu trúc project Cargo ([`Cargo.toml`](./Cargo.toml), [`Cargo.lock`](./Cargo.lock), [`src/main.rs`](./src/main.rs), [`.gitignore`](./.gitignore))
- [x] Nắm quy trình lệnh hàng ngày: `cargo check`, `cargo run`, `cargo fmt`, `cargo clippy` *(riêng `cargo test` sẽ thực hành kỹ ở Chặng 4)*
- [x] Cài `rust-analyzer` trong VS Code *(sẽ dùng xuyên suốt qua các bài tập)*

## Chặng 1: Cú pháp cốt lõi & Tư duy biểu thức (Expression-oriented) *(Hoàn thành)*
- [x] Biến bất biến mặc định (`let` vs `let mut`, `const`) *(áp dụng shadowing khi đọc & parse input)*
- [x] Kiểu dữ liệu cơ bản & tràn số (xem chi tiết tại [`data-types.md`](./data-types.md))
- [x] Hàm: statement vs expression (dấu `;` cuối hàm và block `{ }`)
- [x] Điều khiển luồng: `if` trả về giá trị, `loop`, `while`, `for` với range `1..=10` *(áp dụng `break value;` trong bài tập nhỏ)*
- [x] **Bài tập kết Chặng 1:** Mini-game Đoán số CLI (gộp `const`, `u32`, `shadowing`, `tuple`, `loop` trả giá trị, `std::io`, `parse`)

## Chặng 2: Trái tim của Rust — Ownership, Borrowing & Slices *(Hoàn thành)*
- [x] Stack vs Heap qua cặp đôi `String` và `&str`
- [x] Move semantics: vì sao truyền biến vào hàm xong không dùng lại được?
- [x] Reference & Borrowing: `&T` (shared) vs `&mut T` (exclusive)
- [x] Hiểu và "làm bạn" với thông báo lỗi của Borrow Checker
- [x] Slices (`&str`, `&[i32]`) *(đã thực hành `&str` với `first_word`; `&[T]` sẽ áp dụng cùng `Vec<T>` ở Chặng 4)*

## Chặng 3: Dữ liệu & Trạng thái an toàn — Struct, Enum, Option/Result *(Hoàn thành)*
- [x] `struct` + `impl` (methods `&self`, `&mut self`, `self` và `Self::new`)
- [x] `enum` mang dữ liệu kèm theo
- [x] Pattern matching: `match`, `if let`, `let ... else`
- [x] `Option<T>` (cách Rust thay thế `null`)
- [x] `Result<T, E>` (cách Rust thay thế `try / catch` & Exception)
- [x] **Bài tổng hợp kết Chặng 3:** Máy bán nước tự động mini ([`src/vending_machine.rs`](./src/vending_machine.rs))

## Chặng 4: Collections, Error Handling & Tổ chức code *(Hoàn thành)*
- [x] Buổi 4.1: `Vec<T>`, `HashMap<K, V>` & Entry API ([`src/lesson_4_collections.rs`](./src/lesson_4_collections.rs))
- [x] Buổi 4.2: Closures & Iterators (`.iter()`, `.into_iter()`, `.iter_mut()`, `.map()`, `.filter()`, `.collect()`, `.find()`) ([`src/lesson_4_iterators.rs`](./src/lesson_4_iterators.rs))
- [x] Buổi 4.3: Xử lý lỗi nâng cao & Toán tử `?` (thay cho `.unwrap()`) ([`src/lesson_4_errors.rs`](./src/lesson_4_errors.rs))
- [x] Buổi 4.4: Tổ chức module (`mod`, `pub`, `use`) & Unit test với `#[cfg(test)]`, `cargo test` ([`src/lesson_4_errors.rs`](./src/lesson_4_errors.rs))

## Chặng 5: Traits, Generics & Mini-Project tổng hợp *(Tiếp theo)*
- [ ] `trait` và `#[derive(Debug, Clone, PartialEq)]`
- [ ] Generics `<T>` và trait bounds cơ bản
- [ ] **Mini-Project:** Ứng dụng CLI quản lý công việc / chi tiêu (lưu file JSON bằng `serde`, parse tham số dòng lệnh)
