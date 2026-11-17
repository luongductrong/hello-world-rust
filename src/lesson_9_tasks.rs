pub async fn run() {
    let mut total_bytes = 0_u64;
    let mut handles = Vec::new();
    let start = std::time::Instant::now();
    println!("\n=== BẮT ĐẦU CHẠY 1.000 ASYNC TASKS VỚI TOKIO::SPAWN ===");

    for id in 1..=1_000 {
        let handle = tokio::spawn(async move { simulate_download(id).await });
        handles.push(handle);
    }

    for handle in handles {
        let bytes = handle.await.unwrap();

        total_bytes = total_bytes.saturating_add(bytes);
    }

    let duration = start.elapsed();

    println!("Hoàn thành toàn bộ 1.000 tasks!");
    println!("Tổng số bytes tải được: {total_bytes} bytes");
    println!(
        "Thời gian chạy thực tế: {duration:?} (Cực kỳ khủng khiếp: 1.000 tasks hoàn thành trong tích tắc!)"
    );

    // TEST
    // let local_text = String::from("cục bộ");
    // let slice = &local_text;

    // `local_text` does not live long enough
    // borrowed value does not live long enough

    // tokio::spawn(async move {
    //     println!("{slice}");
    // });
}

async fn simulate_download(task_id: usize) -> u64 {
    let delay = 50 + (task_id as u64 % 50);
    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;

    (task_id as u64) * 10
}
