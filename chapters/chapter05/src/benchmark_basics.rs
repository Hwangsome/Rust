// benchmark 的核心不是“能不能跑”，而是“测量方式是否合理”。
// 这里先用 `Instant` 做最小例子，帮助读者理解粗略计时流程。
use std::time::Instant;

pub fn run() {
    println!("== Benchmark Basics ==");

    // 记录起点时间，再执行一段可重复的工作负载。
    let start = Instant::now();
    let sum: u64 = (1..=10_000).sum();
    let elapsed = start.elapsed();

    println!("sum = {sum}");
    println!("粗略计时结果 = {elapsed:?}");
    println!("更正式的 benchmark 通常会使用 criterion 之类的工具。");
    println!();
}
