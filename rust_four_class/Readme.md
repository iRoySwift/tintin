## 为你自己定义的一个类型或多个类型实现加法运算（用符号 +），并构思使用 Trait Object 实现类型方法的调用。

- 提示：

  - 实现 Add trait
  - 实现一个函数，接受 Trait Object 作为参数

- 基本数据相加

```rust
trait MyAdd {
    fn add(&self) -> i32;
}

impl MyAdd for i32 {
    fn add(&self) -> i32 {
        *self as i32
    }
}

impl MyAdd for f64 {
    fn add(&self) -> i32 {
        *self as i32
    }
}

fn add_object(obj1: Box<dyn MyAdd>, obj2: Box<dyn MyAdd>) -> i32 {
    obj1.add() + obj2.add()
}

fn main() {
    let obj1 = 10;
    let obj2 = 10.1;
    let sum = add_object(Box::new(obj1), Box::new(obj2));
    println!("sum: {:?}", sum);
}

```

- 坐标相加

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn run() {
    assert_eq!(
        Point { x: 1, y: 1 } + Point { x: 2, y: 2 },
        Point { x: 3, y: 3 }
    )
}

```

## 使用枚举包裹三个不同的类型，并放入一个 Vec 中，对 Vec 进行遍历，调用三种不同类型的各自的方法。

```rust
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

```

## 定义三个不同的类型，使用 Trait Object，将其放入一个 Vec 中，对 Vec 进行遍历，调用三种不同类型的各自的方法。 同时，说明其上两种不同实现方法的区别。

- 注意：
  - trait 中不能放置关联函数

```rust
// ## 定义三个不同的类型，使用 Trait Object，将其放入一个 Vec 中，对 Vec 进行遍历，调用三种不同类型的各自的方法。 同时，说明其上两种不同实现方法的区别。

use std::fmt::Debug;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 关联函数
    fn new(x: T, y: T) -> Self {
        Point { x, y }
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
    fn show() {
        println!("show Circle")
    }
}

trait StructVector {
    // 方法
    fn call(&self);
    fn show_associated(&self);
}

impl<T> StructVector for Point<T> {
    // 方法
    fn call(&self) {
        println!("结构体： Point");
    }

    fn show_associated(&self) {
        Point::<T>::show();
        // println!("show Point");
    }
}
impl StructVector for Rectangle {
    fn call(&self) {
        println!("单元结构体： Rectangle");
    }
    fn show_associated(&self) {
        Rectangle::show();
        // println!("show Rectangle")
    }
}
impl<T> StructVector for Circle<T> {
    fn call(&self) {
        println!("元组结构体： Circle");
    }
    fn show_associated(&self) {
        Circle::<T>::show();
        // println!("show Circle")
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
        item.show_associated();
    }
}


```

## 两种不同实现方法的区别

- enum:

  1.  枚举集合可以通过 match 语句来匹配成员，来调用关联函数
  2.  调用 struct 上的方法，也只能通过匹配成员来调用

- trait

  1. trait object 集合无法通过 match 语句来匹配成员，调用关联函数，只能通过 trait 上 show_associated 方法 来调用各个类型上的关联函数 show
  2. 调用 struct 上的方法，可以直接调用
