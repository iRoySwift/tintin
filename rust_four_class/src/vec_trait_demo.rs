// ## 定义三个不同的类型，使用 Trait Object，将其放入一个 Vec 中，对 Vec 进行遍历，调用三种不同类型的各自的方法。 同时，说明其上两种不同实现方法的区别。

use std::fmt::Debug;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: Debug,
{
    // 关联函数
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle;
impl Rectangle {
    fn new() -> Self {
        Rectangle
    }
}

#[derive(Debug)]
struct Circle<T>(T);
impl<T> Circle<T> {
    fn new(radius: T) -> Self {
        Circle(radius)
    }
}

trait StructVector {
    // 方法
    fn call(&self);
    fn show(&self);
}

impl<T> StructVector for Point<T> {
    // 方法
    fn call(&self) {
        println!("结构体： Point");
    }

    fn show(&self) {
        // Point::<T>::show();
        println!("show Point");
    }
}
impl StructVector for Rectangle {
    fn call(&self) {
        println!("单元结构体： Rectangle");
    }
    fn show(&self) {
        // Rectangle::show();
        println!("show Rectangle")
    }
}
impl<T> StructVector for Circle<T> {
    fn call(&self) {
        println!("元组结构体： Circle");
    }
    fn show(&self) {
        // Circle::<T>::show();
        println!("show Circle")
    }
}

// 运行run方法
pub fn run() {
    let vec_enum: Vec<Box<dyn StructVector>> = vec![
        Box::new(Point::new(0, 2)),
        Box::new(Rectangle::new()),
        Box::new(Circle::new(5)),
    ];
    for item in vec_enum {
        item.call();
        // 调用关联函数
        item.show();
    }
}
