//! 第 14 章练习：把并发和异步放在真实场景里练。

pub fn run() {
    println!("== Lab ==");
    println!("▷ 练习 1：spawn + join，拿子线程的返回值");
    println!("▷ 练习 2：用 mpsc 写一个 producer-consumer，生产 10 条消息");
    println!("▷ 练习 3：用 Arc<Mutex<Vec<i32>>> 让 4 个线程并发 push，最后打印长度");
    println!("▷ 练习 4：用 RwLock 做 \"1 写 / 3 读\" 场景，观察比 Mutex 的吞吐差异");
    println!("▷ 练习 5：用 Atomic 代替 Mutex 计数，看看差距");
    println!("▷ 练习 6：用 Barrier 实现 \"所有线程打完 phase-1 才开始 phase-2\"");
    println!("▷ 练习 7：用 thread::scope 写一段借用栈上 Vec 的并行统计");
    println!("▷ 练习 8：手写一个 block_on，跑你自己的 async fn");
    println!("▷ 练习 9：可选：`cargo add tokio --features full` 后改写为 tokio 版本");
    println!();
    println!("完成标准：");
    println!("  - 能选对并发原语：channel / Mutex / RwLock / Atomic / Barrier");
    println!("  - 知道 async 是惰性的，必须有 runtime 驱动");
    println!("  - 遇到 CPU 密集不会盲目塞进 async");
    println!();
}
