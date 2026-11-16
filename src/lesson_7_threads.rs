pub fn run() {
    run_parallel_computation();

    // `Rc<String>` cannot be sent between threads safely
    // within `{closure@src\lesson_7_threads.rs:5:24: 5:31}`, the trait `Send` is not implemented for `Rc<String>`

    // let non_thread_safe = std::rc::Rc::new(String::from("dữ liệu đơn luồng"));
    // std::thread::spawn(move || {
    //     println!("{}", non_thread_safe);
    // });
}

fn process_chunk(worker_id: usize, data: Vec<u64>) -> u64 {
    let mut total = 0_u64;
    let thread_id = std::thread::current().id();

    println!(
        "[Worker {worker_id} trên Thread {thread_id:?}] Bắt đầu xử lý {} phần tử...",
        data.len()
    );
    std::thread::sleep(std::time::Duration::from_millis(100)); // 100ms

    for i in data {
        total = total.saturating_add(i);
    }

    println!("[Worker {worker_id}] Hoàn thành, tổng = {total}");
    total
}

fn run_parallel_computation() {
    let mut total = 0_u64;
    let mut handles = Vec::new();
    // Note giúp tôi cách tạo nhanh Vec
    let data = (1_u64..=100_000).collect::<Vec<_>>();
    // Và cách chia chunks
    let chunks: Vec<&[u64]> = data.chunks(25_000).collect();

    for (id, chunk) in chunks.iter().enumerate() {
        let own_chunk = chunk.to_vec();
        let handle = std::thread::spawn(move || process_chunk(id + 1, own_chunk));

        handles.push(handle);
    }

    for handle in handles {
        total += handle.join().unwrap();
    }

    println!("Tổng nhận được: {total}");
}
