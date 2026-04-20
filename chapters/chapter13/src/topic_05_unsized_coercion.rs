//! Unsized coercion（非固定大小强转）：编译器自动把 Sized 类型的引用变成 Unsized 类型的胖指针。
//!
//! 几个最常见的隐式转换：
//!
//! ```text
//!  &[T; N]         →  &[T]         （数组 → 切片）
//!  &String         →  &str         （自动 deref 后）
//!  Box<ConcreteT>  →  Box<dyn Trait>（具体类型 → trait object）
//!  &ConcreteT      →  &dyn Trait
//! ```
//!
//! 这类转换之所以能发生，是因为原类型**实现了** `CoerceUnsized` trait（标准库内部 trait）。
//! 对日常代码来说你只需要记住："我把固定大小数组的引用传进去，能自动当切片用。"

use std::fmt::Display;

trait Shout {
    fn shout(&self) -> String;
}

impl Shout for i32 {
    fn shout(&self) -> String { format!("{self}!") }
}

impl Shout for String {
    fn shout(&self) -> String { format!("{}!!!", self.to_uppercase()) }
}

fn print_slice(xs: &[i32]) {
    println!("  slice = {xs:?}");
}

fn print_display(x: &dyn Display) {
    println!("  display = {x}");
}

pub fn run() {
    println!("== Unsized Coercion ==");

    println!("-- (1) &[T; N] 自动变 &[T] --");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // 这里 `&arr` 是 `&[i32; 5]`（Sized），被隐式强转成 `&[i32]`（Unsized 的胖指针）
    print_slice(&arr);
    println!();

    println!("-- (2) &String → &str （Deref 强转）--");
    let s = String::from("hello");
    // `&String` 隐式变 `&str`，上一章已经见过
    let _ = s.len();
    println!("  见 chapter02 的 deref 强转");
    println!();

    println!("-- (3) &T → &dyn Trait --");
    let n: i32 = 42;
    let st: String = "rust".into();
    print_display(&n);  // &i32 → &dyn Display
    print_display(&st); // &String → &dyn Display
    println!();

    println!("-- (4) Box<ConcreteT> → Box<dyn Trait> --");
    let boxed: Box<dyn Shout> = Box::new(7_i32); // 从 Box<i32> 隐式变 Box<dyn Shout>
    println!("  {}", boxed.shout());
    let boxed2: Box<dyn Shout> = Box::new(String::from("rust"));
    println!("  {}", boxed2.shout());
    println!();
}
