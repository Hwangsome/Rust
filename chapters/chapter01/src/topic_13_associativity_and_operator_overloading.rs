//! 结合性与运算符重载。
//!
//! 这一节要同时讲两件事：
//!
//! 1. **结合性（associativity）**：当多个**同级**运算符连续出现时，按什么顺序分组。
//!    - 大多数二元运算是**左结合**：`20 - 5 - 3` 相当于 `(20 - 5) - 3`
//!    - 赋值、复合赋值是**右结合**：`a = b = 1` 相当于 `a = (b = 1)`（不过 Rust 的 `=` 是语句，不返回值）
//!
//! 2. **运算符重载（operator overloading）**：通过实现 `std::ops::*` 里的 trait，
//!    让**自定义类型**也能使用 `+`、`-`、`*` 这些运算符。这本质上是在告诉 Rust
//!    "当看到 `Point + Point` 时，该按哪种规则计算"。
//!
//! 运行后请重点对照：
//! - `20 - 5 - 3` vs `20 - (5 - 3)`：证明减法是**左结合**
//! - `p1 + p2` 能调用 `Point::add`：证明我们成功"重载"了 `+`
//! - `-p`：只要实现 `Neg` 就能让自定义类型支持一元 `-`

use std::ops::{Add, Mul, Neg};

/// 一个最小的"平面坐标点"结构体。
/// 用 `#[derive(...)]` 给它加上打印、复制、相等性比较的能力——详见第 10 节。
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/// 为 `Point` 实现 `Add`：让 `p1 + p2` 能用。
///
/// - `type Output = Self;` 表示加法结果仍然是 `Point`
/// - `self` 是左操作数，`rhs` 是右操作数（right-hand side）
/// - 实际相当于：`p1 + p2` ≡ `Add::add(p1, p2)` ≡ `p1.add(p2)`
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// 为 `Point` 实现 `Neg`：让 `-p` 能用（一元负号）。
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

/// 为 `Point` 实现"标量乘法"：`p * 2` 得到一个每个分量都乘 2 的新点。
///
/// 注意：`impl Mul<i32> for Point` 表示"右操作数是 `i32`"——这就允许我们重载 `Point * i32`。
/// 要让 `i32 * Point` 也合法，还得再实现 `impl Mul<Point> for i32`（本节略）。
impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

/// 1) 结合性：减法 **左结合**，所以去掉括号默认从左往右分组。
fn demonstrate_left_associativity() {
    let default_grouping = 20 - 5 - 3; // 等价于 (20 - 5) - 3
    let forced_grouping = 20 - (5 - 3);
    println!("20 - 5 - 3      = {default_grouping} (默认按 (20-5)-3 计算)");
    println!("20 - (5 - 3)    = {forced_grouping}");
}

/// 2) 先乘除后加减：这其实是**优先级**问题，和"结合性"是两件事。
///
/// - 优先级（precedence）：**不同**运算符之间，谁先算
/// - 结合性（associativity）：**相同**运算符连续出现时，怎么分组
fn demonstrate_precedence_vs_associativity() {
    let by_precedence = 2 + 3 * 4; // 3 * 4 先算 -> 2 + 12
    let left_assoc_same_level = 20 - 5 - 3; // 同级，左结合
    println!("2 + 3 * 4        = {by_precedence} (优先级: * 优于 +)");
    println!("20 - 5 - 3       = {left_assoc_same_level} (结合性: 左结合)");
}

/// 3) 运算符重载：让自定义类型 `Point` 支持 `+`、`-`、`* i32`。
fn demonstrate_operator_overloading() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    let sum = p1 + p2; // 调用 Add::add
    let neg = -p1;     // 调用 Neg::neg
    let scaled = p2 * 3; // 调用 Mul::mul

    println!("p1 + p2       = {sum:?}");
    println!("-p1           = {neg:?}");
    println!("p2 * 3        = {scaled:?}");
}

pub fn run() {
    println!("-- (1) 结合性: 减法是左结合 --");
    demonstrate_left_associativity();
    println!();

    println!("-- (2) 优先级 vs 结合性 --");
    demonstrate_precedence_vs_associativity();
    println!();

    println!("-- (3) 运算符重载: impl Add/Neg/Mul for Point --");
    demonstrate_operator_overloading();
    println!();
}
