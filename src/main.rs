// mod primitives;

use ferris_says::{self, say};
use std::io::{BufWriter, stdout};
fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    function_reference();

    promitives()
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

// Promitives 基本数据结构
fn promitives() {
    // 
    let logical: bool = true;
    println!("{}", logical);

    // 支持前后标注变量类型
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    println!("{}, {}", a_float, an_integer);

    // 也支持类型推断
    let default_float = 3.0; // f64
    let default_integer = 7; // i32
    println!("{}，{}", default_float, default_integer);

    let mut inferred_type = 12; // 默认推断为 i32
    // inferred_type = 4294967296; // 不使用尾标注 i64 会报错
    inferred_type = 429467296i64;
    println!("{}", inferred_type);

    let mut mutable = 12;
    mutable = 21;
    println!("{}", mutable);

    // mutable = true; // 会报错，变量的类型不能改变
    let mutable = true; // 变量可以通过重复声明（重写）来更新类型和值

    println!("{}", mutable);
}