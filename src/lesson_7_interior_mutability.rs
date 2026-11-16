use std::cell::RefCell;
use std::sync::{Mutex, RwLock};

pub fn run() {
    println!("=== PHẦN 1: RefCell Logger ===");
    let logger = LocalLogger::new();
    logger.log("Khởi tạo");
    logger.log("Kết thúc");
    logger.print_logs();

    // thread 'main' (17168) panicked at src\lesson_7_interior_mutability.rs:24:19:
    // RefCell already borrowed
    // => Chủ động đóng scope sớm
    {
        let _r = logger.logs.borrow();
    }
    logger.log("test");

    println!("\n=== PHẦN 2: Mutex Metrics ===");
    let metrics = SafeMetrics::new();
    metrics.increment();
    metrics.increment();
    metrics.increment();
    metrics.increment();
    println!("Metrics Count sau khi tăng: {}", metrics.get_count());

    println!("\n=== PHẦN 3: RwLock App Config ===");
    let config = AppConfig::new("Light Mode");
    println!("Theme ban đầu: {}", config.get_theme());

    // Deadlock luồng ghi nếu 4 luồng đọc không kết thúc (vì đang ở main thread)
    {
        let _t1 = config.theme.read().unwrap();
        let _t2 = config.theme.read().unwrap();
        let _t3 = config.theme.read().unwrap();
        let _t4 = config.theme.read().unwrap();
    }

    config.update_theme("Dark Mode");
    println!("Theme sau cập nhật: {}", config.get_theme());
}

struct LocalLogger {
    logs: RefCell<Vec<String>>,
}

struct SafeMetrics {
    request_count: Mutex<u64>,
}

struct AppConfig {
    theme: RwLock<String>,
}

impl LocalLogger {
    fn new() -> Self {
        Self {
            logs: RefCell::new(Vec::new()),
        }
    }
    fn log(&self, msg: &str) {
        self.logs.borrow_mut().push(msg.to_string());
    }
    fn print_logs(&self) {
        // self.logs.borrow() trả về Ref<Vec<T>> chứ không phải Vec<T>
        for (i, log) in self.logs.borrow().iter().enumerate() {
            println!("[RefCell] Log {}: {log}", i + 1);
        }
    }
}

impl SafeMetrics {
    fn new() -> Self {
        Self {
            request_count: Mutex::new(0),
        }
    }
    fn increment(&self) {
        let mut guard = self.request_count.lock().unwrap();
        *guard += 1;
    }
    fn get_count(&self) -> u64 {
        let guard = self.request_count.lock().unwrap();
        *guard
    }
}

impl AppConfig {
    fn new(default_theme: &str) -> Self {
        Self {
            theme: RwLock::new(default_theme.to_string()),
        }
    }
    fn get_theme(&self) -> String {
        self.theme.read().unwrap().to_string()
    }
    fn update_theme(&self, new_theme: &str) {
        let mut guard = self.theme.write().unwrap();

        *guard = new_theme.to_string();
    }
}
