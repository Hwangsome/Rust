# 6. 共享状态（Part 2）：`RwLock` 与 Atomic

> - **所属章节**：第 14 章 · Concurrency
> - **代码位置**：`chapters/chapter14/src/topic_06_sharing_states_part_2.rs`
> - **上一篇**：[5. 共享状态（part 1）](./5-共享状态part1.md)
> - **下一篇**：[7. Barrier 同步](./7-Barrier同步.md)
> - **关键词**：`RwLock`、`AtomicUsize`、无锁计数、读多写少、`Ordering`

---

## 读多写少：`RwLock<T>`

```rust
use std::sync::{Arc, RwLock};
use std::thread;

let cache = Arc::new(RwLock::new(vec![1, 2, 3]));

// 多个读者并行
let readers: Vec<_> = (0..3).map(|i| {
    let c = Arc::clone(&cache);
    thread::spawn(move || {
        let data = c.read().unwrap(); // 读锁（可以多个并发）
        println!("  读 {i}: {:?}", *data);
    })
}).collect();
for h in readers { h.join().unwrap(); }

// 写者独占
let c = Arc::clone(&cache);
let writer = thread::spawn(move || {
    let mut data = c.write().unwrap(); // 写锁（独占）
    data.push(4);
});
writer.join().unwrap();
```

## 无锁计数：Atomic

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

let counter = Arc::new(AtomicUsize::new(0));

let handles: Vec<_> = (0..100).map(|_| {
    let c = Arc::clone(&counter);
    std::thread::spawn(move || {
        c.fetch_add(1, Ordering::Relaxed); // 原子加，无需 Mutex
    })
}).collect();
for h in handles { h.join().unwrap(); }
println!("计数: {}", counter.load(Ordering::Relaxed)); // 100
```

---

## Mutex vs RwLock vs Atomic 选型

| 场景 | 推荐 |
|-----|-----|
| 任意可变状态 | `Mutex<T>` |
| 读多写少 | `RwLock<T>` |
| 简单计数/标志 | `Atomic*` |
| 消息传递 | `channel` |

---

## 下一步

- 继续阅读：[7. Barrier 同步](./7-Barrier同步.md)
