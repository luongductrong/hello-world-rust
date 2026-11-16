pub fn run() {
    let text_worker = TextWorker {
        content: "Xin chào Rust".to_string(),
    };
    let num_worker = ComputeWorker { number: 6 };

    println!("{}", run_worker(text_worker));
    println!("{}", run_worker(num_worker));

    let job = Box::new(PrintJob {
        text: "Rust xin chào".to_string(),
    });
    // Lần này tôi không cần khai báo kiểu của job như bài dyn Trait hôm trước mà để compiler tự suy luận là Box<PrintJob>
    // dispatch_job nhận vào Box<dyn ThreadSafeJob + Send + Sync> vẫn truyền Box<PrintJob> bình thường?
    dispatch_job(job);
}

trait Worker {
    type Response;

    fn perform(&self) -> Self::Response;
    fn name(&self) -> &str;
}

trait ThreadSafeJob {
    fn run_job(&self);
}

struct TextWorker {
    content: String,
}
struct ComputeWorker {
    number: u64,
}

struct PrintJob {
    text: String,
}

impl Worker for TextWorker {
    type Response = String;

    fn name(&self) -> &str {
        "Bộ xử lý văn bản"
    }
    fn perform(&self) -> Self::Response {
        self.content.to_uppercase()
    }
}

impl Worker for ComputeWorker {
    type Response = u64;

    fn name(&self) -> &str {
        "Bộ tính toán số học"
    }
    fn perform(&self) -> Self::Response {
        let mut sum = 0_u64;

        for i in 1..self.number {
            sum = sum.saturating_add(i);
        }
        sum.saturating_add(self.number)
    }
}

impl ThreadSafeJob for PrintJob {
    fn run_job(&self) {
        println!("{}", self.text);
    }
}

fn run_worker<W>(worker: W) -> W::Response
where
    W: Worker,
{
    println!("{}", worker.name());
    worker.perform()
}

fn dispatch_job(job: Box<dyn ThreadSafeJob + Send + Sync>) {
    job.run_job();
}
