#![allow(dead_code)]

#[derive(Debug)] // 对结构体进行标记，才能使用 print 方法打印内容
pub struct Person {
    pub(crate) name: String,
    pub(crate) age: u8
}

#[derive(Debug)]
pub struct Unit;

#[derive(Debug)]
pub struct Pair(pub(crate) i32, pub(crate) f32);

#[derive(Debug)]
pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub(crate) top_left: Point,
    pub(crate) bottom_right: Point
}