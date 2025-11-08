use ferris_says::{say};
use std::io::{stdout, BufWriter, Write};

fn main() {
    hello_world();
    f_say();
}

fn hello_world() {
    let greeting: &str = "Hello, World!"; 
    println!("{}", greeting);

    let mut out = stdout();
    writeln!(out, "{}", greeting).unwrap();
}

fn f_say() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock()); 
    say(&message, width, &mut writer).unwrap();
}