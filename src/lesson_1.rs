// Biến, Kiểu dữ liệu & Hàm
pub fn run() {
    const MAX: i32 = 100_000;

    let mut x = 5;
    let y = x;

    println!("Value of x is: {}", x);
    println!("Value of y is: {}", y);

    x = 6;

    println!("Value of x is: {}", x);
    println!("Value of y is: {}", y);

    println!("MAX is: {}", MAX);

    let mut sum = add(3, 4);
    println!("Sum of 3 and 4 is: {}", sum);

    sum = add(sum, 5);
    println!("Sum of {} and 5 is: {}", sum - 5, sum);

    let celsius = 37.0;
    let fahrenheit = c_to_f(celsius);
    println!("{celsius} C is {fahrenheit:.1} F");

    let temp = celsius + 1.0;
    let status = if temp > 37.5 {
        // điều kiện bắt buộc là bool
        "Sốt"
    } else {
        "Bình thường"
    };

    println!("Nhiệt độ: {temp}, Trạng thái: {status}");

    hello_5_times();
    sorry_10_times();
    hey_3_times();
    what_7_times();
    check_temps();
    mock_shadowing();

    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b // KHÔNG có ';' => đây là Expression
}

fn c_to_f(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn hello_5_times() {
    // 0, 1, 2, 3, 4
    // for i in 0..5 {
    //     println!("Hello, {i}");
    // }
    // 1, 2, 3, 4, 5
    for i in 1..=5 {
        println!("Hello, {i}");
    }
}

fn what_7_times() {
    let temps = [1, 2, 3, 4, 5, 6, 7];

    for temp in temps {
        println!("What?: {temp}");
    }
}

fn sorry_10_times() {
    let mut i = 0;
    loop {
        if i >= 10 {
            break;
        }
        println!("Sorry, {i}");
        i += 1;
    }
}

fn hey_3_times() {
    let mut i = 0;
    while i < 3 {
        println!("Hey, {i}");
        i += 1;
    }
}

fn analyze_temp(celsius: f64) -> (f64, bool) {
    let f = celsius * 1.8 + 32.0;
    let is_fever = celsius > 37.5;
    (f, is_fever)
    // tuple
}

fn check_temps() {
    let temps = [36.5, 37.0, 38.2, 39.1];

    for t in temps {
        let (f, is_fever) = analyze_temp(t);

        let status = if is_fever {
            // điều kiện bắt buộc là bool
            "Sốt"
        } else {
            "Bình thường"
        };

        println!("{t}°C => {f:.1}°F ({status})"); // tránh thiếu dấu ";"
    }
}

fn mock_shadowing() {
    let input = "  38.2 \n"; // kiểu &str
    let input = input.trim(); // vẫn tên `input`, bỏ khoảng trắng
    let input: f64 = input.parse().unwrap(); // lưu ý khi dùng unwrap

    println!("Input: {input}")
}
