use anyhow::{Context, Result, bail, ensure};

pub fn run() {
    fn matcher(input: Result<()>) {
        if let Err(err) = input {
            println!("\n{err:#}");
        } else {
            println!("\nGiao dịch hoàn tất thành công!");
        }
    }

    matcher(handle_checkout_command(
        "ghost_user.json",
        "cart-001",
        70_000,
        100_000,
    ));
    matcher(handle_checkout_command(
        "data.json",
        "cart-001",
        70_000,
        100_000,
    ));
    matcher(handle_checkout_command(
        "data-2.json",
        "cart-001",
        200_000,
        50_000,
    ));
    matcher(handle_checkout_command(
        "data-2.json",
        "cart-001",
        200_000,
        500_000,
    ));
}

fn load_user_profile(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Lỗi khi mở file hồ sơ tại: '{path}'"))?;

    ensure!(
        !content.trim().is_empty(), // điều kiện ngược, nếu true, cho qua, nếu false, ném lỗi
        "Hồ sơ người dùng rỗng, không có dữ liệu"
    );
    Ok(content)
}

fn checkout_cart(cart_id: &str, total_amount: u64, balance: u64) -> Result<()> {
    ensure!(
        total_amount > 0,
        "Giỏ hàng '{cart_id}' không có sản phẩm nào để thanh toán"
    );
    if balance < total_amount {
        bail!(
            "Thanh toán thất bại cho giỏ hàng '{cart_id}': thiếu {}đ (ví có {balance}đ, cần {total_amount}đ)",
            total_amount - balance
        );
    }
    Ok(())
}

fn handle_checkout_command(
    profile_path: &str,
    cart_id: &str,
    amount: u64,
    balance: u64,
) -> Result<()> {
    load_user_profile(profile_path).context("Bước 1: Tải thông tin người dùng thất bại")?;
    checkout_cart(cart_id, amount, balance).context("Bước 2: Xử lý giao dịch giỏ hàng thất bại")?;

    Ok(())
}
