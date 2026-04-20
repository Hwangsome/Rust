//! `thread::scope`：**作用域线程**——子线程可以借用父作用域的变量。
//!
//! 普通 `thread::spawn` 要求 `F: 'static`，所以不能借用栈上局部。
//! `thread::scope(|s| { s.spawn(...) })` 保证所有子线程在 scope 结束**之前**都被 join，
//! 因此可以安全借用父栈上的数据——**不需要 Arc / clone**。

use std::thread;

pub fn run() {
    println!("== Scoped Threads ==");

    let numbers = vec![10, 20, 30, 40];

    thread::scope(|s| {
        // 子线程可以直接借用 numbers
        s.spawn(|| {
            let sum: i32 = numbers.iter().sum();
            println!("  [scoped t1] sum = {sum}");
        });
        s.spawn(|| {
            let max = numbers.iter().max().unwrap_or(&0);
            println!("  [scoped t2] max = {max}");
        });
        // scope 结束前 Rust 自动 join 所有子线程
    });

    // 现在所有子线程都已 join，numbers 仍然是主线程的
    println!("  main still owns numbers = {numbers:?}");
    println!();
}
