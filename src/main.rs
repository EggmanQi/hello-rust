use ferris_says::{self, say};
use std::io::{BufWriter, stdout};
fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    function_reference();
}

fn function_reference() {
    let s = String::from("hello");

    // 引用 `String::from()` 函数
    let from_string = String::from;
    println!("s: {}", s);

    // 使用引用函数
    let s2 = from_string("world");
    println!("s2: {}", s2);
}
