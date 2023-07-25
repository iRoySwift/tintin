// ## 使用枚举包裹三个不同的类型，并放入一个 Vec 中，对 Vec 进行遍历，调用三种不同类型的各自的方法。

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
    // 方法
    fn call(self) {
        println!("结构体： Point");
    }
    fn show() {
        println!("show Point")
    }
}

#[derive(Debug)]
struct Rectangle;
impl Rectangle {
    fn new() -> Self {
        Rectangle
    }
    fn call(self) {
        println!("单元结构体： Rectangle");
    }
    fn show() {
        println!("show Rectangle")
    }
}

#[derive(Debug)]
struct Circle<T>(T);
impl<T> Circle<T> {
    fn new(radius: T) -> Self {
        Circle(radius)
    }
    fn call(self) {
        println!("元组结构体： Circle");
    }
    fn show() {
        println!("show Circle")
    }
}

#[derive(Debug)]
enum Graphics<T> {
    Point(Point<T>),
    Rectangle(Rectangle),
    Circle(Circle<T>),
}

// 运行run方法
pub fn run() {
    let vec_enum = vec![
        Graphics::Point(Point::new(0, 2)),
        Graphics::Rectangle(Rectangle::new()),
        Graphics::Circle(Circle::new(5)),
    ];
    for item in vec_enum {
        match item {
            Graphics::Point(p) => {
                p.call();
                Point::<u32>::show();
            }
            Graphics::Rectangle(r) => {
                r.call();
                Rectangle::show()
            }
            Graphics::Circle(c) => {
                c.call();
                Circle::<u32>::show()
            }
        };
    }
}
