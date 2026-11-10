### Bài 0: Làm chủ Cargo & quy trình code Rust hàng ngày

1. **`cargo check`** — *Kiểm tra code nhanh nhất (dùng liên tục lúc đang viết)*
2. **`cargo run`** (và `cargo build`) — *Biên dịch và chạy*
   * `cargo build`: dịch code thành file thực thi `target\debug\hello-world.exe` (chế độ dev, dịch nhanh để debug). Khi cần bản tối ưu tốc độ để phát hành thì dùng `cargo build --release` (nằm trong `target\release\`).
   * `cargo run`: tự động build nếu code có sửa rồi chạy luôn file `.exe` đó.

3. **`cargo fmt`** — *Tự động làm đẹp code theo chuẩn Rust*
   * `cargo fmt`: tự căn thụt đầu dòng, khoảng trắng, sắp xếp `use` trực tiếp vào file.
   * `cargo fmt --check`: chỉ kiểm tra xem file có đúng chuẩn format chưa (không sửa file).

4. **`cargo clippy`** — *Trợ lý bắt "code mùi" & gợi ý viết chuẩn Rust*

5. **`cargo test`** & **`cargo add`** *(sẽ dùng ở các chặng sau)*
   * `cargo test`: chạy các bài test tự động trong project.
   * `cargo add <tên_crate>`: cài thêm thư viện ngoài vào [`Cargo.toml`](./Cargo.toml) (không cần mở file gõ tay).

**Nhịp làm việc chuẩn khi mở VS Code:**
Gõ code (nhìn gợi ý/báo đỏ từ `rust-analyzer`) → Lưu (`Ctrl + S` tự `fmt`) → `cargo clippy` / `cargo check` → `cargo run` để kiểm chứng kết quả.