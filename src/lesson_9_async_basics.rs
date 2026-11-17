pub async fn run() {
    println!("\n=== PHẦN 1: Lazy Future ===");
    let fut = lazy_task(99);

    println!("--> Đã tạo future 99 nhưng chưa gọi .await!");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let result = fut.await;

    println!("Kết quả nhận được: {result}");

    println!("\n=== PHẦN 2: tokio::join! Dashboard ===");
    (load_dashboard().await).unwrap();
}

async fn lazy_task(id: u32) -> String {
    println!("[Task {id}] Bắt đầu chạy...");
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    format!("Kết quả task {id}")
}

async fn fetch_user_name() -> anyhow::Result<String> {
    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
    Ok("Luong Duc Trong".to_string())
}

async fn fetch_notifications_count() -> anyhow::Result<u32> {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok(12)
}

async fn fetch_wallet_balance() -> anyhow::Result<u64> {
    tokio::time::sleep(std::time::Duration::from_millis(120)).await;
    Ok(25_000_000)
}

async fn load_dashboard() -> anyhow::Result<()> {
    let start = std::time::Instant::now();

    let (user_res, notif_res, balance_res) = tokio::join!(
        fetch_user_name(),
        fetch_notifications_count(),
        fetch_wallet_balance()
    );

    let user = user_res?;
    let notif = notif_res?;
    let balance = balance_res?;

    let duration = start.elapsed();

    println!("--- DASHBOARD ---");
    println!("Người dùng: {user}\nThông báo chưa đọc: {notif}\nSố dư ví: {balance}đ");
    println!("Thời gian tải dashboard: {duration:?} (Nhanh gấp đôi so với chạy tuần tự 370ms!)");

    Ok(())
}
