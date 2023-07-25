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

pub fn run() {
    let obj1 = 10;
    let obj2 = 10.1;
    let sum = add_object(Box::new(obj1), Box::new(obj2));
    println!("sum: {:?}", sum);
}
