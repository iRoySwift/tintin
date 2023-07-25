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
