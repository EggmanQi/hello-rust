use ferris_says::{self, say};
use std::io::{BufWriter, stdout};

///
/// 参考内容
/// https://doc.rust-lang.org/rust-by-example
/// https://rusty.course.rs/
/// 

// 引用其他文件的方式
mod structures;
use crate::structures::{Point, Rectangle, Unit, Pair};

const _MAX_POINT: u32 = 100_000;

fn main() {
    the_first_case();

    function_reference();
    learn_promitives();

    learn_structures();
}

fn the_first_case() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
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

struct Struct {
    e: i32
}
// Promitives 基本数据结构
fn learn_promitives() {
    // 变量绑定概念 - logical 拥有 ‘true’ 内存的所有权
    // 没有 var 的概念，所有变量默认不可变，需要通过 mut 修饰变量使其可变
    let _logical: bool = true;
    // println!("{}", logical);

    // 支持前后标注变量类型
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    println!("{}, {}", a_float, an_integer);

    // 也支持类型推断
    let default_float = 3.0; // f64
    let default_integer = 7; // i32
    println!("{}，{}", default_float, default_integer);

    let mut inferred_type = 12; // 默认推断为 i32
    println!("{}", inferred_type);
    // inferred_type = 4294967296; // 不使用尾标注 i64 会报错
    inferred_type = 429467296i64;
    println!("{}", inferred_type);

    let mut mutable = 12;
    println!("{}", mutable);
    mutable = 21;
    println!("{}", mutable);

    // mutable = true; // 会报错，变量的类型不能改变
    let mutable = true; // 变量可以通过重复声明（重写）来更新类型和值
    println!("{}", mutable);

    // 对复杂变量进行解构
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    // 进行更复杂的结构赋值
    let (a,b,c,d,e);
    (a, b) = (1, 2); // 元组
    [c, .., d, _] = [1, 2, 3, 4, 6]; // 切片
    Struct {e, ..} = Struct {e: 5}; // 结构题
    assert_eq!([1,2,3,6,0], [a,b,c,d,e]);
}

fn learn_structures() {
    let name = String::from("Peter");
    let age = 23;

    let peter = structures::Person {
        name, age
    };
    println!("{:?}",peter);

    let point: Point = Point { x: 10.3, y: 0.4};
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    println!("the rectangle is: {:?}", _rectangle);

    let _unit = Unit;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}