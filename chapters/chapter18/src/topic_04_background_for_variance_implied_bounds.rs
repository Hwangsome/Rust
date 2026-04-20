//! 方差背景：**隐式 bound**（implied bounds）。
//!
//! 当你写 `&'a T`，Rust 会自动推断一个**隐式 bound**：`T: 'a`（T 里的引用都活得至少 `'a`）。
//!
//! 这避免了你在每个函数签名里都要写 `where T: 'a` 的样板。
//!
//! ```ignore
//! fn f<'a, T>(x: &'a T) { ... }
//! // 等价于: fn f<'a, T: 'a>(x: &'a T) { ... }
//! ```

use std::fmt::Debug;

fn show<'a, T: Debug>(x: &'a T) {
    // Rust 隐式知道 T: 'a
    println!("  {x:?}");
}

pub fn run() {
    println!("== Variance Background: Implied Bounds ==");

    let v = vec![1, 2, 3];
    show(&v);

    let s = String::from("rust");
    show(&s);
    println!();

    println!("  写 `&'a T` 时 Rust 自动加 T: 'a 约束");
    println!();
}
