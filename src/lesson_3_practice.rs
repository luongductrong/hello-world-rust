pub fn run() {
    let mut t1 = ProjectTask::new(1, "Giả vờ học bài");
    let mut t2 = ProjectTask::new(2, "Ngáp");
    let mut t3 = ProjectTask::new(3, "Đi ngủ");

    t1.work(30, 199);
    t1.block("thấy khó coi quá");
    t1.unblock();
    t1.work(40, 139);

    t2.work(50, 400);
    t2.block("hơi ngại");
    t2.unblock();
    t2.work(100, 400);

    t3.work(60, 10);
    t3.block("tỉnh cả ngủ vì thấy bug");
    t3.unblock();
    t3.work(70, 12);

    let tasks = [t1, t2, t3];

    for task in &tasks {
        task.summary();
    }

    let total_minutes = ProjectTask::total_done_minutes(&tasks);

    println!("Tổng số phút đã hoàn thành: {total_minutes} (phút)");

    if let Some(task) = ProjectTask::find_by_id(&tasks, 2) {
        // Tôi dùng if let ở đây vì ngại return ở hàm chính
        task.summary();
    }

    match ProjectTask::find_by_id(&tasks, 99) {
        None => println!("Có cái nịt!"),
        Some(task) => task.summary(),
    }

    let maybe_note = Some(String::from("Nhớ commit code"));
    if let Some(text) = &maybe_note {
        println!("Note: {text}");
    }
    println!("Debug: {maybe_note:?}");
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
            Self::Todo => String::from("Từ từ tao làm, đừng có hối"),
            Self::Blocked(reason) => format!("Tự nhiên {reason} nên không làm"),
            Self::InProgress { progress_pct } => {
                format!("Bình tĩnh, làm được {progress_pct}% rồi, đừng vội")
            }
            Self::Done { minutes } => format!("Xời, {minutes} phút là làm xong ngay, tưởng gì khó"),
        }
    }

    fn advance(&mut self, add_pct: u8, minutes_if_done: u32) {
        match self {
            Self::Todo => {
                let pct = add_pct.min(100);
                if pct == 100 {
                    *self = Self::Done {
                        minutes: minutes_if_done,
                    };
                } else {
                    *self = Self::InProgress { progress_pct: pct };
                }
            }
            Self::InProgress { progress_pct } => {
                let new_pct = progress_pct.saturating_add(add_pct).min(100);
                if new_pct == 100 {
                    *self = Self::Done {
                        minutes: minutes_if_done,
                    };
                } else {
                    *progress_pct = new_pct;
                }
            }
            Self::Blocked(_) | Self::Done { .. } => {
                println!("Chịu, không tăng được nữa");
            }
        }
    }
}

#[derive(Debug)]
struct ProjectTask {
    id: u32,
    title: String,
    status: TaskStatus,
}

impl ProjectTask {
    fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            status: TaskStatus::Todo,
        }
    }

    fn work(&mut self, add_pct: u8, minutes_if_done: u32) {
        self.status.advance(add_pct, minutes_if_done);
    }

    fn block(&mut self, reason: &str) {
        self.status = TaskStatus::Blocked(reason.to_string());
    }

    fn unblock(&mut self) {
        let TaskStatus::Blocked(_) = &self.status else {
            return;
        };
        self.status = TaskStatus::Todo;
    }

    fn summary(&self) {
        println!("#{} [{}]: {}", self.id, self.title, self.status.describe());
    }

    fn find_by_id(tasks: &[ProjectTask], target_id: u32) -> Option<&ProjectTask> {
        // for task in tasks {
        //     if task.id == target_id {
        //         return Some(task);
        //     }
        // }
        // None
        // Hoặc dùng hàm có sẵn
        tasks.iter().find(|task| task.id == target_id)
    }

    fn done_minutes(&self) -> Option<u32> {
        let TaskStatus::Done { minutes } = self.status else {
            return None;
        };
        Some(minutes)
    }

    fn total_done_minutes(tasks: &[ProjectTask]) -> u32 {
        let mut total = 0;

        for task in tasks {
            total += task.done_minutes().unwrap_or(0);
        }
        total
    }
}
