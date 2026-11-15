// MINI-PROJECT CAPSTONE
use serde::{Deserialize, Serialize};
use std::{env, fmt::Display, fs, path::Path};

const DEFAULT_FILE_NAME: &str = "data.json";

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let Ok(mut task_manager) = TaskManager::load(DEFAULT_FILE_NAME) else {
        println!("Không thể tải dữ liệu");
        return;
    };

    // Vừa học được: map không chỉ dành cho Vec, mà Result, Option cũng có map
    match args.get(1).map(|s| s.as_str()) {
        Some("add") => add_task(&args, &mut task_manager),
        Some("remove") | Some("delete") => delete_task(&args, &mut task_manager),
        Some("done") => done_task(&args, &mut task_manager),
        Some("list") => list_tasks(&task_manager),
        _ => println!("Lệnh không hợp lệ"),
    }

    let Ok(_) = task_manager.save() else {
        println!("Không thể lưu dữ liệu");
        return;
    };

    // Ban đầu tôi khai báo Vec<&str> nhưng lỗi, nhờ cứu viện, rồi sửa thành &[String]
    // Bạn giải thích lại giúp nhé
    fn add_task(args: &[String], manager: &mut TaskManager) {
        let Some(title) = args.get(2) else {
            println!("Không đủ thông tin để tạo nhiệm vụ mới");
            return;
        };
        if title.trim().is_empty() {
            println!("Không đủ thông tin để tạo nhiệm vụ mới");
            return;
        }
        let Ok(_) = manager.add(title.clone()) else {
            println!("Không thể tạo nhiệm vụ, danh sách đã đầy");
            return;
        };
        println!("Tạo mới thành công. Nhiệm vụ: {title}");
    }
    fn delete_task(args: &[String], manager: &mut TaskManager) {
        let Some(text_id) = args.get(2) else {
            println!("Không đủ thông tin để tạo nhiệm vụ mới");
            return;
        };
        if text_id.trim().is_empty() {
            println!("Thiếu ID nhiệm vụ");
            return;
        }
        let id: u32 = match text_id.parse() {
            Ok(id) => id,
            Err(_) => {
                println!("ID không hợp lệ");
                return;
            }
        };
        let Ok(task_id) = manager.delete(id) else {
            println!("Xoá thất bại: Không tìm thấy nhiệm vụ");
            return;
        };
        println!("Đã xoá thành công: Nhiệm vụ #{task_id}");
    }
    fn done_task(args: &[String], manager: &mut TaskManager) {
        let Some(text_id) = args.get(2) else {
            println!("Không đủ thông tin để tạo nhiệm vụ mới");
            return;
        };
        if text_id.trim().is_empty() {
            println!("Thiếu ID nhiệm vụ");
            return;
        }
        let id: u32 = match text_id.parse() {
            Ok(id) => id,
            Err(_) => {
                println!("ID không hợp lệ");
                return;
            }
        };
        let Ok(task) = manager.mark_done(id) else {
            println!("Cập nhật thất bại: Nhiệm vụ đã hoàn thành trước đó");
            return;
        };
        println!("Cập nhật thành công:\n {task}");
    }
    fn list_tasks(manager: &TaskManager) {
        println!("{}", manager.list());
    }

    // Hơi boilerplate nhưng mới học nên cũng tạm chấp nhận được
    // Chưa tận dụng được hết AppError
}

enum AppError {
    TaskNotFound,
    ListTooLong,
    FileReader,
    JsonParser,
    NoChanges,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

struct TaskManager {
    items: Vec<Task>,
    file_path: String,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}. {} ({})",
            self.id,
            self.title,
            if self.done {
                "Đã hoàn thành"
            } else {
                "Chưa hoàn thành"
            }
        )
    }
}

impl TaskManager {
    fn load(file_path: &str) -> Result<Self, AppError> {
        let file_exists = Path::new(file_path).exists();

        if file_exists {
            let content = fs::read_to_string(file_path).map_err(|_| AppError::FileReader)?;
            let tasks: Vec<Task> =
                serde_json::from_str(&content).map_err(|_| AppError::JsonParser)?;

            Ok(Self {
                items: tasks,
                file_path: file_path.to_string(),
            })
        } else {
            Ok(Self {
                items: Vec::new(),
                file_path: file_path.to_string(),
            })
        }
    }

    fn save(&self) -> Result<(), AppError> {
        let content =
            serde_json::to_string_pretty(&self.items).map_err(|_| AppError::JsonParser)?;

        fs::write(&self.file_path, content).map_err(|_| AppError::FileReader)?;

        Ok(())
    }

    fn add(&mut self, title: String) -> Result<(), AppError> {
        let mut current_max_id: u32 = 0;
        for item in &self.items {
            if item.id > current_max_id {
                current_max_id = item.id;
            }
        }
        // Code hiện tại đi theo hướng không cấp phát lại id cũ, trừ khi items bị xoá sạch
        // Chưa tối ưu nhưng tạm chấp nhận được
        if current_max_id > u32::MAX - 1 {
            return Err(AppError::ListTooLong);
        }

        let new_task = Task {
            id: current_max_id + 1,
            title,
            done: false,
        };

        self.items.push(new_task);

        Ok(())
    }

    fn list(&self) -> String {
        if self.items.is_empty() {
            return "Chưa có dữ liệu".to_string();
        }

        let header = "Danh sách công việc:\n";
        let body = self
            .items
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<_>>()
            .join("\n");

        format!("{header}{body}")
    }

    fn mark_done(&mut self, task_id: u32) -> Result<Task, AppError> {
        // Không cần let mut để sửa task vì bản thân task đã là ref &mut Task
        // Code nhiều tôi mới nhận ra cái này
        let task = self
            .items
            .iter_mut()
            .find(|item| item.id == task_id)
            .ok_or(AppError::TaskNotFound)?;

        if task.done {
            Err(AppError::NoChanges)
        } else {
            task.done = true;
            Ok(task.clone())
        }
    }

    fn delete(&mut self, task_id: u32) -> Result<u32, AppError> {
        let Some(index) = self.items.iter().position(|item| item.id == task_id) else {
            return Err(AppError::TaskNotFound);
        };

        self.items.remove(index);
        Ok(task_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut manager = TaskManager {
            items: Vec::new(),
            file_path: DEFAULT_FILE_NAME.to_string(),
        };

        assert!(matches!(manager.add("Task 01".to_string()), Ok(_)));
        assert!(manager.list().contains("1. Task 01"));
    }

    #[test]
    fn test_remove_task() {
        let mut manager = TaskManager {
            items: Vec::new(),
            file_path: DEFAULT_FILE_NAME.to_string(),
        };

        assert!(matches!(manager.add("Task 01".to_string()), Ok(_)));
        assert!(matches!(manager.delete(1), Ok(1)));
    }
}
