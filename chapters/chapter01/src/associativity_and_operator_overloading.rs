// 这一节同时演示两个概念：
// 1. 结合性会影响同优先级表达式怎样“抱团”
// 2. Rust 的运算符重载通常通过 trait 实现，而不是随意魔改语法
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    // 实现 `Add` 之后，`Point + Point` 才能成立。
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub fn run() {
    println!("== Associativity And Operator Overloading ==");

    let without_parentheses = 20 - 5 - 3;
    let with_parentheses = 20 - (5 - 3);
    println!("20 - 5 - 3 = {without_parentheses}");
    println!("20 - (5 - 3) = {with_parentheses}");

    // 这里的 `+` 不再是内建数字加法，而是走我们上面实现的 `Add` 逻辑。
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let sum = p1 + p2;
    println!("operator overloading with Add trait => {:?}", sum);
    println!();
}
