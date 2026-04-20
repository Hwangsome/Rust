//! tokio 任务（本章不引入依赖，**用文字描述**替代真实运行）。
//!
//! 为什么不直接加 tokio？教学仓库坚持"`cargo run` 能一键跑通、依赖最少"。
//! 本节给出 tokio 的最小心智模型 + 代码片段示例。
//!
//! ## 心智模型
//!
//! - **Runtime**（运行时）：`#[tokio::main]` 宏在程序启动时创建一个 tokio runtime
//! - **Task**：`tokio::spawn(async { ... })` 把一个 future 扔给 runtime，类似"轻量线程"
//!   - 与 OS 线程不同：一个 runtime 可能只有几个 worker OS 线程，但同时跑成千上万个 task
//! - **await points**：task 在 await 时让出 worker 线程，让别的 task 先跑
//!
//! ## 最小示例（需要 `tokio` 依赖）
//!
//! ```ignore
//! #[tokio::main]
//! async fn main() {
//!     let t1 = tokio::spawn(async { /* do work 1 */ });
//!     let t2 = tokio::spawn(async { /* do work 2 */ });
//!     let _ = tokio::join!(t1, t2);
//! }
//! ```
//!
//! ## 任务 vs 线程
//!
//! | 维度 | OS 线程 | tokio task |
//! |-----|--------|-----------|
//! | 创建成本 | 高 | 极低（几 KB 栈） |
//! | 并发量 | 几百~几千 | 几十万 |
//! | 阻塞 API | 直接写 | 禁止——用 async API 或 `spawn_blocking` |

pub fn run() {
    println!("== Tokio Tasks (concept) ==");
    println!("  tokio::spawn 把 Future 交给 runtime 调度");
    println!("  `#[tokio::main]` 宏自动创建 runtime 并驱动 main");
    println!("  tokio::join! 并发等待多个任务");
    println!("  不要在 async 代码里调用阻塞 API——用 tokio::fs / tokio::net / spawn_blocking");
    println!("  本章不引入 tokio 依赖；代码片段见文件顶部注释");
    println!();
}
