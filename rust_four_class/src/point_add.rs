// 坐标相加
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

fn add_object<T: Add<Output = T>>(obj1: T, obj2: T) -> T {
    obj1 + obj2
}

pub fn run() {
    assert_eq!(
        add_object(Point { x: 1, y: 1 }, Point { x: 2, y: 2 }),
        Point { x: 3, y: 3 }
    )
}
