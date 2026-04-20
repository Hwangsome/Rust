# 5. 共享状态（Part 1）：`Arc<Mutex<T>>`

> - **所属章节**：第 15 章 · Concurrency
> - **代码位置**：`chapters/chapter15/src/topic_05_sharing_states_part_1.rs`
> - **上一篇**：[4. 消息传递（part 2）](./4-消息传递part2.md)
> - **下一篇**：[6. 共享状态（part 2）](./6-共享状态part2.md)
> - **关键词**：`Arc<Mutex<T>>`、多线程共享、`lock()`、`MutexGuard`、RAII 解锁

---

## 多线程共享可变状态的标准组合

```
Arc<T>：原子引用计数，允许多个线程"拥有"同一份数据
Mutex<T>：互斥锁，同一时刻只有一个线程可以访问内部数据

Arc<Mutex<T>> = 多线程共享 + 可变访问
```

---

## 基础用法

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..10).map(|_| {
    let c = Arc::clone(&counter);
    thread::spawn(move || {
        let mut guard = c.lock().unwrap(); // 获取锁（阻塞直到可用）
        *guard += 1;
        // guard drop 时自动释放锁（RAII）
    })
}).collect();

for h in handles { h.join().unwrap(); }
println!("计数: {}", *counter.lock().unwrap()); // 10
```

---

## 完整运行示例

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    println!("=== Arc<Mutex<T>> ===");
    let data = Arc::new(Mutex::new(Vec::<i32>::new()));

    let handles: Vec<_> = (0..5).map(|i| {
        let d = Arc::clone(&data);
        thread::spawn(move || {
            let mut guard = d.lock().unwrap();
            guard.push(i * i);
        })
    }).collect();

    for h in handles { h.join().unwrap(); }

    let result = data.lock().unwrap();
    let mut sorted = result.clone();
    sorted.sort();
    println!("  收集到的平方数（排序后）: {sorted:?}");
}
```

---

## 死锁预防

```
死锁发生条件：线程 A 持有锁 1，等待锁 2；
              线程 B 持有锁 2，等待锁 1。

预防策略：
  1. 始终以相同顺序获取多把锁
  2. 持锁时间尽量短
  3. 持锁期间不要调用可能再获取相同锁的代码
```

---

## 下一步

- 继续阅读：[6. 共享状态（part 2）](./6-共享状态part2.md)
