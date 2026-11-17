pub fn run() {
    fn matcher(input: Result<u64, OrderError>) {
        match &input {
            Ok(total) => println!("\nĐặt hàng thành công, tổng thanh toán là {total}đ"),
            Err(err) => println!("\n{err}"),
        }
        if let Err(OrderError::InsufficientFunds { .. }) = input {
            println!("[Gợi ý UI]: Vui lòng nạp thêm tiền!");
        }
    }

    matcher(process_order("abc", 100, 200, 100_000_000, None));
    matcher(process_order(
        "100",
        100,
        200,
        100_000_000,
        Some("non_existent_config.json"),
    ));
    matcher(process_order("100", 0, 200, 100_000_000, None));
    matcher(process_order("10000", 10, 2, 10_000, None));
    matcher(process_order("10000", 10, 200, 10_000, None));
    matcher(process_order("50000", 10, 20, 100_000_000, None));
}

#[derive(thiserror::Error, Debug)]
enum OrderError {
    #[error("Số lượng đặt hàng không hợp lệ: {0} (phải lớn hơn 0)")]
    InvalidQuantity(u32),

    #[error("Sản phẩm '{item_name}' không đủ tồn kho (chỉ còn {available} món)")]
    OutOfStock { item_name: String, available: u32 },

    #[error("Số dư không đủ: có {balance}đ, cần thanh toán {cost}đ")]
    InsufficientFunds { balance: u64, cost: u64 },

    #[error("Lỗi đọc cấu hình đơn hàng: {0}")]
    ConfigError(#[from] std::io::Error),

    #[error("Lỗi parse đơn giá sản phẩm: {0}")]
    ParsePriceError(#[from] std::num::ParseIntError),
}

fn process_order(
    price_str: &str,
    quantity: u32,
    stock: u32,
    balance: u64,
    config_path: Option<&str>,
) -> Result<u64, OrderError> {
    let order_price = price_str.parse::<u64>()?;

    if let Some(config_path) = config_path {
        let config = std::fs::read_to_string(config_path)?;
        println!("Config gọi cho vui chứ không liên quan tới nghiệp vụ hả: {config}");
    }

    if quantity == 0 {
        return Err(OrderError::InvalidQuantity(quantity));
    }

    if quantity > stock {
        return Err(OrderError::OutOfStock {
            item_name: "Bro cho đề gì lạ thế, item_name đâu mà dùng?".to_string(),
            available: stock,
        });
    }

    let total = order_price.saturating_mul(quantity as u64);

    if balance < total {
        return Err(OrderError::InsufficientFunds {
            balance,
            cost: total,
        });
    }

    Ok(total)
}
