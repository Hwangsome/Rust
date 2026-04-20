# 12. CPU 密集任务

> - **所属章节**：第 15 章 · Concurrency
> - **代码位置**：`chapters/chapter15/src/topic_12_computationally_expensive_functions.rs`
> - **上一篇**：[11. tokio 任务](./11-tokio任务.md)
> - **下一篇**：[13. 并发抓取项目](./13-并发抓取项目.md)

---

## IO 密集 vs CPU 密集

```
IO 密集（等待磁盘/网络）：
  → async/await + tokio
  → 一个线程服务大量并发连接

CPU 密集（纯计算）：
  → OS 线程 / rayon
  → 充分利用多核

混合场景（在 tokio 中有 CPU 密集部分）：
  → spawn_blocking 把 CPU 工作转到独立线程池
```

---

## `spawn_blocking`：避免阻塞 tokio 线程

```rust
// ❌ 不好：在 async 里做 CPU 密集，阻塞 tokio worker
async fn bad() {
    let result = compute_heavy(10_000_000); // 阻塞！
}

// ✅ 好：转到专用线程池
async fn good() {
    let result = tokio::task::spawn_blocking(|| {
        compute_heavy(10_000_000)
    }).await.unwrap();
}
```

---

## 并行计算用 rayon

```rust
// Cargo.toml: rayon = "1"
use rayon::prelude::*;

let data: Vec<i64> = (0..1_000_000).collect();
let sum: i64 = data.par_iter().sum(); // 自动并行！
```

---

## 完整运行示例

```rust
use std::thread;
use std::time::Instant;

fn compute(n: u64) -> u64 { (0..n).filter(|x| x % 7 == 0).sum() }

pub fn run() {
    let n = 1_000_000u64;

    // 顺序
    let start = Instant::now();
    let r1 = compute(n);
    let r2 = compute(n);
    println!("顺序: {:?}", start.elapsed());

    // 并行
    let start = Instant::now();
    let (h1, h2) = (
        thread::spawn(move || compute(n)),
        thread::spawn(move || compute(n)),
    );
    let r1 = h1.join().unwrap();
    let r2 = h2.join().unwrap();
    println!("并行: {:?} (应约一半时间)", start.elapsed());
}
```

---

## 下一步

- 继续阅读：[13. 并发抓取项目](./13-并发抓取项目.md)
