use std::sync::{Arc, Mutex, RwLock};

pub fn run() {
    println!("===== Bank Account =====");
    let account = BankAccount::new("VCB-888888", 1_000_000_u64);
    let account_state = Arc::new(Mutex::new(account));

    println!("Count: {}", Arc::strong_count(&account_state));
    let mut handles = Vec::new();

    for _ in 1..=5 {
        let state_clone = Arc::clone(&account_state);

        let handle = std::thread::spawn(move || {
            let thread_id = std::thread::current().id();
            std::thread::sleep(std::time::Duration::from_millis(50));
            let mut guard = state_clone.lock().unwrap();
            guard.deposit(100_000, "Thưởng KPI");

            println!(
                "[Thread {thread_id:?}] Giao dịch thành công, số dư tạm thời: {}đ",
                guard.balance
            );
        });
        handles.push(handle);
    }
    // 1 tí boilerplate nhưng kệ đi
    for _ in 1..=5 {
        let state_clone = Arc::clone(&account_state);

        let handle = std::thread::spawn(move || {
            let thread_id = std::thread::current().id();
            std::thread::sleep(std::time::Duration::from_millis(50));
            let mut guard = state_clone.lock().unwrap();
            guard.withdraw(50_000, "Uống cà phê");

            println!(
                "[Thread {thread_id:?}] Giao dịch thành công, số dư tạm thời: {}đ",
                guard.balance
            );
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("===== System Status =====");
    let system_state = Arc::new(RwLock::new(SystemStatus {
        is_maintenance: false,
        active_users: 100,
    }));

    let mut handles = Vec::new();

    for _ in 1..=4 {
        let state_clone = Arc::clone(&system_state);

        let handle = std::thread::spawn(move || {
            let thread_id = std::thread::current().id();

            let guard = state_clone.read().unwrap();
            println!(
                "[Reader {thread_id:?}] Hệ thống bảo trì: {}.",
                guard.is_maintenance
            );
        });
        handles.push(handle);
    }
    let state_clone = Arc::clone(&system_state);
    let handle = std::thread::spawn(move || {
        let mut guard = state_clone.write().unwrap();
        guard.is_maintenance = true;
        guard.active_users = 0;
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("===== Nghiệm thu =====");
    println!("Count: {}", Arc::strong_count(&account_state));

    let guard = account_state.lock().unwrap();
    println!(
        "Tài khoản: {}, số dư cuối: {}đ",
        guard.account_number, guard.balance
    );

    for trans in guard.transactions.iter() {
        println!("{trans}");
    }

    let guard = system_state.read().unwrap();
    println!(
        "Bảo trì: {}, Người dùng: {}",
        guard.is_maintenance, guard.active_users
    );
}

struct BankAccount {
    account_number: String,
    balance: u64,
    transactions: Vec<String>,
}

impl BankAccount {
    fn new(account_number: &str, initial_balance: u64) -> Self {
        Self {
            account_number: account_number.to_string(),
            balance: initial_balance,
            transactions: Vec::new(),
        }
    }
    fn deposit(&mut self, amount: u64, note: &str) {
        self.balance = self.balance.saturating_add(amount);
        self.transactions.push(format!("+{amount}đ: {note}"));
    }
    fn withdraw(&mut self, amount: u64, note: &str) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            self.transactions.push(format!("-{amount}đ: {note}"));
            true
        } else {
            false
        }
    }
}

struct SystemStatus {
    is_maintenance: bool,
    active_users: u64,
}
