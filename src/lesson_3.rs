pub fn run() {
    let mut task = Task::new(1, "Học struct Rust");

    println!("{task:#?}");

    task.print_info();
    task.toggle();
    task.print_info();

    let title = task.finish_and_take_title();

    println!("{title}");

    // task.print_info();

    let statuses = [
        TaskStatus::Todo,
        TaskStatus::InProgress { progress_pct: 50 },
        TaskStatus::Blocked(String::from("Lười")),
        TaskStatus::Done { minutes: 1726 },
    ];

    for status in &statuses {
        print!("{}", status.describe());

        if let TaskStatus::Blocked(reason) = status {
            print!("  -> Cảnh báo: {reason}");
        }

        TaskStatus::print_if_done(status);
    }
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

impl Task {
    fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            done: false,
        }
    }

    fn print_info(&self) {
        println!(
            "[{}] #{}: {}",
            if self.done { "x" } else { " " },
            self.id,
            self.title
        );
    }

    fn toggle(&mut self) {
        self.done = !self.done;
    }

    fn finish_and_take_title(self) -> String {
        self.title
    }
}

#[derive(Debug)]
enum TaskStatus {
    Todo,
    InProgress { progress_pct: u8 },
    Blocked(String),
    Done { minutes: u32 },
}

impl TaskStatus {
    fn describe(&self) -> String {
        match self {
            Self::Todo => String::from("Chưa làm"),
            Self::InProgress { progress_pct } => format!("Đang làm ({progress_pct}%)"),
            Self::Blocked(reason) => format!("Kẹt: {reason}"),
            Self::Done { minutes } => format!("Đã xong trong {minutes} phút"),
        }
    }

    fn print_if_done(status: &TaskStatus) {
        let Self::Done { minutes } = status else {
            println!("  -> Chưa hoàn thành");
            return;
        };

        println!("  -> Đã tốn {minutes} phút");
    }
}
