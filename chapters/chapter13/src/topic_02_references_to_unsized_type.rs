//! 对 Unsized 类型的引用：**胖指针**。
//!
//! Unsized 类型（`str`, `[T]`, `dyn Trait`）不能按值传递，但可以**通过引用**使用：
//!
//! ```text
//!  &str        = 指针 + 长度 byte 数
//!  &[T]        = 指针 + 元素个数
//!  &dyn Trait  = 指针 + vtable 指针（方法表）
//!  Box<dyn T>  = 同上，但堆上独占
//! ```
//!
//! 长度/vtable 叫"元数据"（metadata），让运行时能知道这个引用背后到底有多长/有哪些方法。

use std::mem::size_of_val;

trait Greeting {
    fn greet(&self) -> String;
}

struct En;
struct Fr;

impl Greeting for En { fn greet(&self) -> String { "hello".into() } }
impl Greeting for Fr { fn greet(&self) -> String { "bonjour".into() } }

pub fn run() {
    println!("== References to Unsized Type ==");

    println!("-- (1) &str 胖指针 --");
    let s: &str = "hello, 世界";
    println!("  值的长度 size_of_val(s) = {} bytes", size_of_val(s));
    println!("  引用本身 sizeof(&str) 见上一节");
    println!();

    println!("-- (2) &[i32] 胖指针 --");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a;
    println!("  slice 元素个数 = {}", slice.len());
    println!("  size_of_val(slice) = {} bytes (5 * 4)", size_of_val(slice));
    println!();

    println!("-- (3) &dyn Trait 胖指针 --");
    // 同一个引用类型 `&dyn Greeting` 可以指向 `En` 或 `Fr`：vtable 负责区分。
    let langs: [&dyn Greeting; 2] = [&En, &Fr];
    for g in langs.iter() {
        println!("  dynamic dispatch -> {}", g.greet());
    }
    println!();
}
