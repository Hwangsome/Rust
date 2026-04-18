//! 基准测试基础：从 `Instant` 粗略计时到 `criterion` 严肃测量。
//!
//! Rust 的 benchmark 生态分两层：
//!
//! 1. **原生 `#[bench]`**：只在 **nightly** 上可用（`#![feature(test)]`），不推荐
//! 2. **`criterion` crate**：稳定版 Rust 上事实标准的基准测试框架
//!    - 统计分析（均值、标准差、异常点剔除）
//!    - 自动做热身（warm-up）
//!    - 生成 HTML 报告
//!
//! 本节只演示"粗略计时"——用 `std::time::Instant` 自测一段代码大概多快。
//! 真要做严肃测量，去看 `criterion` 的文档。
//!
//! ## 自测计时的 3 个常见坑
//!
//! 1. **Debug 模式**的结果**毫无参考价值**：Rust debug build 会关掉几乎所有优化。
//!    用 `cargo run --release -p chapter05` 才有意义。
//! 2. **编译器常量折叠**：如果输入是常量，编译器可能直接把结果算出来，测出 `0ns`。
//!    解决：用 `std::hint::black_box(...)` 或让输入来自运行时参数。
//! 3. **单次测量噪声大**：最好跑多次取中位数；冷启动的第一次要丢掉。

use std::hint::black_box;
use std::time::Instant;

/// 一个可重复的负载：累加从 1 到 N。
fn workload(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..=n {
        // black_box 告诉编译器"你不能把这里优化掉"，
        // 这样常量输入下的基准才有意义。
        sum = sum.wrapping_add(black_box(i));
    }
    sum
}

pub fn run() {
    println!("== Benchmark Basics ==");

    // 做 5 次，去掉第一次（冷启动），打印剩下 4 次的结果。
    let n: u64 = 1_000_000;
    let mut samples = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        let sum = workload(black_box(n));
        let elapsed = start.elapsed();
        samples.push((sum, elapsed));
    }

    println!("workload(1..={n}) 的计时样本（第一次常含冷启动噪声）:");
    for (i, (sum, elapsed)) in samples.iter().enumerate() {
        println!("  run #{i}: sum = {sum}, time = {elapsed:?}");
    }
    println!();

    println!("-- 自测计时的注意事项 --");
    println!("  1. 用 cargo run --release 才有参考价值，debug 模式结果失真");
    println!("  2. 用 std::hint::black_box 阻止编译器把常量结果直接算出来");
    println!("  3. 多跑几次、去掉冷启动；要严肃测量就上 criterion crate");
    println!();
}
