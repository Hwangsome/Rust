# 7. Barrier：让所有线程在同一点同步

> - **所属章节**：第 14 章 · Concurrency
> - **代码位置**：`chapters/chapter14/src/topic_07_synchronization_through_barriers.rs`
> - **上一篇**：[6. 共享状态（part 2）](./6-共享状态part2.md)
> - **下一篇**：[8. 作用域线程](./8-作用域线程.md)

---

## Barrier：等齐了才继续

适合"分阶段并行"：所有线程完成 phase 1，才一起进入 phase 2。

```rust
use std::sync::{Arc, Barrier};
use std::thread;

let n = 4;
let barrier = Arc::new(Barrier::new(n));

let handles: Vec<_> = (0..n).map(|i| {
    let b = Arc::clone(&barrier);
    thread::spawn(move || {
        println!("  [{i}] phase 1 完成");
        b.wait(); // 等待所有线程到达
        println!("  [{i}] phase 2 开始");
    })
}).collect();

for h in handles { h.join().unwrap(); }
```

**输出示例**（顺序不确定）：

```text
[0] phase 1 完成
[2] phase 1 完成
[1] phase 1 完成
[3] phase 1 完成
（等所有线程都到达后）
[0] phase 2 开始
[3] phase 2 开始
...
```

---

## 下一步

- 继续阅读：[8. 作用域线程](./8-作用域线程.md)
