//! 函数项强转：**fn item** 可以变 **fn pointer**。
//!
//! Rust 里每个具名函数 `fn add(...)` 都有一个**独特的、零大小的类型**（called "fn item"）：
//!
//! ```text
//! fn add(a: i32, b: i32) -> i32   ← 这是一个 fn 声明
//! add                              ← 但 `add` 本身的类型是独特的 fn item 类型，size = 0
//! ```
//!
//! 在需要 `fn(i32, i32) -> i32`（fn pointer）的位置，编译器自动把 fn item 强转成 fn pointer。
//! 代价：fn pointer 不是零大小（typically 8 字节），但能统一多个函数。

use std::mem::size_of_val;

fn add(a: i32, b: i32) -> i32 { a + b }
fn sub(a: i32, b: i32) -> i32 { a - b }

pub fn run() {
    println!("== Function Item Coercion ==");

    println!("-- (1) fn item 本身大小为 0 --");
    println!("  size_of_val(&add) = {}", size_of_val(&add));
    println!();

    println!("-- (2) fn item → fn pointer --");
    let ops: [fn(i32, i32) -> i32; 2] = [add, sub];
    //     ^^^^^^^^^^^^^^^^^^^^^^^^^^ 数组要求统一类型 → fn pointer
    for op in ops.iter() {
        println!("  op(10, 3) = {}", op(10, 3));
    }
    println!("  fn pointer 大小 = {}", size_of_val(&ops[0]));
    println!();

    println!("-- (3) 调用 fn item vs fn pointer --");
    // 直接调用 add(1, 2)：走 fn item 路径，编译器能内联
    // 通过 ops[0](1, 2) 调用：走 fn pointer 路径，运行期间接调用
    println!("  add(1, 2)   = {}", add(1, 2));
    println!("  ops[0](1,2) = {}", ops[0](1, 2));
    println!();
}
