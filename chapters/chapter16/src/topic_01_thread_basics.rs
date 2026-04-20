//! 线程基础：`std::thread::spawn` / `join` / `sleep`。
//!
//! Rust 的线程 = **OS 线程**（由操作系统调度；和 Go 的 goroutine、Erlang 的绿色线程不一样）。
//! 所有线程 API 都在 `std::thread`：
//!
//! - `thread::spawn(|| { ... })`：启动一个线程，返回 `JoinHandle<T>`
//! - `JoinHandle::join()`：阻塞等待该线程结束，返回 `Result<T, ...>`
//! - `thread::sleep(Duration::from_millis(n))`：当前线程睡眠
//! - `thread::current().id()` / `.name()`：查看当前线程身份

use std::thread;
use std::time::Duration;

pub fn run() {
    println!("== Thread Basics ==");

    println!("-- (1) 最小 spawn --");
    let handle = thread::spawn(|| {
        println!("  [child] hello from thread {:?}", thread::current().id());
    });
    handle.join().expect("child panic");
    println!();

    println!("-- (2) 子线程返回值 --");
    let handle = thread::spawn(|| -> i32 { 7 * 6 });
    let result = handle.join().expect("join failed");
    println!("  child returned = {result}");
    println!();

    println!("-- (3) sleep + join 顺序 --");
    let h1 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(20));
        println!("  [t1] awake");
    });
    let h2 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        println!("  [t2] awake");
    });
    h1.join().ok();
    h2.join().ok();
    println!("  main continues after both joined");
    println!();
}
