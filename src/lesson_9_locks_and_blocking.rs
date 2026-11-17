use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

pub async fn run() {
    let mut handles = Vec::new();
    let session_state = Arc::new(AsyncSessionManager::new());

    for i in 1..=5 {
        let session_clone = Arc::clone(&session_state);

        let handle = tokio::spawn(async move {
            session_clone
                .register_session(format!("User_{i}").as_str())
                .await;
        });

        handles.push(handle);
    }

    for handle in handles {
        // handle chạy tuần tự vì register_session giữ guard qua await
        handle.await.unwrap();
    }

    let ticker = tokio::spawn(async {
        for i in 1..=5 {
            tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
            println!("[UI/EventLoop] Timestep {i} - Vẫn phản hồi cực mượt!");
        }
    });

    let compute_handle = tokio::task::spawn_blocking(move || heavy_hash_simulation(9999));

    let (ticket_res, compute_res) = tokio::join!(ticker, compute_handle);

    ticket_res.unwrap();
    println!("Compute: {}", compute_res.unwrap());
}

struct AsyncSessionManager {
    session_tokens: TokioMutex<Vec<String>>,
}

impl AsyncSessionManager {
    fn new() -> Self {
        Self {
            session_tokens: TokioMutex::new(Vec::new()),
        }
    }
    async fn register_session(&self, user: &str) {
        let mut guard = self.session_tokens.lock().await;

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        guard.push(format!("TOKEN_{user}"));

        println!(
            "[Auth] Đăng ký thành công cho {user}, tổng sessions: {}",
            guard.len()
        );
    }
}

fn heavy_hash_simulation(seed: u64) -> u64 {
    let start = std::time::Instant::now();
    let mut sum = seed;

    for i in 1..=10_000_000 {
        sum = sum.wrapping_add(i);
    }

    let duration = start.elapsed();
    println!("Heavy duration: {duration:?}");
    sum
}
