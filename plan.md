# Lộ trình học Rust qua thực hành (Interactive Roadmap)

> Phương pháp: Học qua **trao đổi + viết code chạy thử ngay**, mỗi khái niệm gắn với tình huống thực tế và thông báo lỗi của compiler. Không đọc sách chay từ đầu đến cuối.

## Chặng 0: Bộ công cụ Cargo & Môi trường *(Đang học)*
- [ ] Xử lý lỗi Windows `os error 4551` (Smart App Control / Developer Mode) để chạy được `cargo run`
- [ ] Hiểu cấu trúc project Cargo ([`Cargo.toml`](./Cargo.toml), [`Cargo.lock`](./Cargo.lock), [`src/main.rs`](./src/main.rs), `.gitignore`)
- [ ] Thành thạo vòng lặp lệnh hàng ngày: `cargo check`, `cargo run`, `cargo fmt`, `cargo clippy`, `cargo test`
- [ ] Tận dụng `rust-analyzer` trong VS Code (inlay hints, quick fix, hover type)

## Chặng 1: Cú pháp cốt lõi & Tư duy biểu thức (Expression-oriented)
- [ ] Biến bất biến mặc định (`let` vs `let mut`) và shadowing
- [ ] Kiểu dữ liệu cơ bản (`i32`, `u32`, `f64`, `bool`, `char`, tuple, array)
- [ ] Hàm: statement vs expression (dấu `;` cuối hàm)
- [ ] Điều khiển luồng: `if` trả về giá trị, `loop`, `while`, `for` với range `1..=10`
- [ ] **Bài tập nhỏ:** Game đoán số hoặc CLI chuyển đổi đơn vị / tính toán đơn giản

## Chặng 2: Trái tim của Rust — Ownership, Borrowing & Slices
- [ ] Stack vs Heap qua cặp đôi `String` và `&str`
- [ ] Move semantics: vì sao truyền biến vào hàm xong không dùng lại được?
- [ ] Reference & Borrowing: `&T` (shared) vs `&mut T` (exclusive)
- [ ] Hiểu và "làm bạn" với thông báo lỗi của Borrow Checker
- [ ] Slices (`&str`, `&[i32]`)

## Chặng 3: Dữ liệu & Trạng thái an toàn — Struct, Enum, Option/Result
- [ ] `struct` + `impl` (methods `&self`, `&mut self`, `self` và `Self::new`)
- [ ] `enum` mang dữ liệu kèm theo
- [ ] `Option<T>` (cách Rust thay thế `null`) & `Result<T, E>`
- [ ] Pattern matching: `match`, `if let`, `let ... else`

## Chặng 4: Collections, Error Handling & Tổ chức code
- [ ] `Vec<T>`, `HashMap<K, V>`, iterator cơ bản (`.iter()`, `.map()`, `.filter()`)
- [ ] Xử lý lỗi chuẩn với toán tử `?` (thay cho `.unwrap()`)
- [ ] Tách module (`mod`, `pub`, `use`)
- [ ] Viết unit test với `#[cfg(test)]` và `cargo test`

## Chặng 5: Traits, Generics & Mini-Project tổng hợp
- [ ] `trait` và `#[derive(Debug, Clone, PartialEq)]`
- [ ] Generics `<T>` và trait bounds cơ bản
- [ ] **Mini-Project:** Ứng dụng CLI quản lý công việc / chi tiêu (lưu file JSON bằng `serde`, parse tham số dòng lệnh)
