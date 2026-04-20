//! 运算符重载：通过实现 `std::ops::*` trait 自定义 `+ - * / % & | ^ << >> ! -`。

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
struct Vec2 { x: f64, y: f64 }

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Vec2 { Vec2 { x: self.x + rhs.x, y: self.y + rhs.y } }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f64) -> Vec2 { Vec2 { x: self.x * s, y: self.y * s } }
}

pub fn run() {
    println!("== Operator Overloading ==");

    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };
    println!("  a + b    = {:?}", a + b);
    println!("  a * 2.5  = {:?}", a * 2.5);
    println!();
}
