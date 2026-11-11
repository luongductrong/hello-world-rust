// Ownership, Borrowing & Slices

pub fn run() {
    // Đoạn A
    let a = String::from("hello");
    let b = a.clone();
    println!("{a} - {b}");

    // Đoạn B
    fn print_len(s: &str) {
        println!("len = {}", s.len());
    }

    let mut msg = String::from("Rust");
    let mut num = 5;

    print_len(&msg);
    print_len("Hello Rust!");
    println!("msg = {msg}");

    fn append_bang(s: &mut String) {
        // s.push_str("!");
        s.push('!');
    }

    append_bang(&mut msg);
    println!("msg = {msg}");

    fn add_ten(n: &mut i32) {
        *n += 10;
    }

    add_ten(&mut num);
    println!("num = {num}");

    fn first_word(s: &str) -> &str {
        for (i, ch) in s.char_indices() {
            if ch == ' ' {
                return &s[..i];
            }
        }
        s
    }

    let mut sentence = String::from("Xin chào Rust");
    let word = first_word(&sentence);

    println!("Từ đầu: {word}");
    sentence.clear();
}
