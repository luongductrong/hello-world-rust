use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run() {
    const MAX_TRIES: u32 = 7;
    let secret = random_1_to_100();
    let mut tries: u32 = 0;

    let (won, tries) = loop {
        let input = read_line_trimmed("Đoán số từ 1 đến 100: ");

        if input == "q" {
            break (false, tries);
        }

        let input: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Vui lòng nhập số nguyên từ 1 đến 100!");
                continue;
            }
        };

        if !(1..=100).contains(&input) {
            println!("Vui lòng nhập số nguyên từ 1 đến 100!");
            continue;
        }

        tries += 1;

        if input == secret {
            break (true, tries);
        }

        if input > secret {
            println!("Quá lớn!");
        } else {
            println!("Quá nhỏ!");
        }

        if tries >= MAX_TRIES {
            break (false, tries);
        }
    };

    let status = if won { "thắng" } else { "thua" };

    println!("Bạn {status} sau {tries} lần chơi!");
    if !won {
        println!("Bật mí: Đáp án là {secret}");
    }
}

fn random_1_to_100() -> u32 {
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_micros();
    (micros % 100) + 1
}

fn read_line_trimmed(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Không đọc được dữ liệu");
    input.trim().to_string()
}
