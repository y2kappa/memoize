#[macro_use]
extern crate memoize;
#[macro_use]
extern crate lazy_static;

#[memoize]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[memoize]
fn mul(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    for _ in 0..3 {
        println!("{:?}", add(100, 200));
        println!("{:?}", mul(3, 4));
    }
}
