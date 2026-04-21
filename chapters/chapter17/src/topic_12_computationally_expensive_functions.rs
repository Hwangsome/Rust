//! CPU 密集型任务：**线程池 vs async**。
//!
//! 关键区别：
//!
//! - **IO 密集**（等待网络 / 磁盘）：用 `async` + `tokio` —— 一个 worker 线程能服务成千上万个连接
//! - **CPU 密集**（纯计算）：用**线程池**（`std::thread` / `rayon`） —— 因为 CPU 时间跑不开就是跑不开
//!
//! 常见误区：把大循环 / 重计算直接写在 `async fn` 里——会占满 worker 线程，导致其他 task 饿死。
//!
//! 正确做法：
//! 1. 在 tokio 环境里：`tokio::task::spawn_blocking(|| cpu_work())`
//! 2. 不在 async 语境：直接 `std::thread` 或引入 `rayon` 做数据并行

use std::thread;
use std::time::Instant;

/// 模拟一个"CPU 密集"的函数：累加平方。
fn heavy_work(upto: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=upto {
        sum = sum.wrapping_add(i * i);
    }
    sum
}

pub fn run() {
    println!("== CPU-bound Functions ==");

    let n: u64 = 200_000;

    println!("-- (1) 单线程顺序跑 2 次 --");
    let start = Instant::now();
    let a = heavy_work(n);
    let b = heavy_work(n);
    println!("  results = ({a}, {b}), elapsed = {:?}", start.elapsed());
    println!();

    println!("-- (2) 起两个线程并行跑 --");
    let start = Instant::now();
    let h1 = thread::spawn(move || heavy_work(n));
    let h2 = thread::spawn(move || heavy_work(n));
    let a = h1.join().unwrap();
    let b = h2.join().unwrap();
    println!("  results = ({a}, {b}), elapsed = {:?} (期待约一半)", start.elapsed());
    println!();

    println!("-- 记忆 --");
    println!("  CPU 密集 → std::thread / rayon；");
    println!("  IO 密集 → async/tokio；");
    println!("  混合场景 → 在 tokio 里用 spawn_blocking 把 CPU 部分挪到阻塞线程池");
    println!();
}
