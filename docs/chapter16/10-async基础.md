# 10. async/await 基础

> - **所属章节**：第 15 章 · Concurrency
> - **代码位置**：`chapters/chapter16/src/topic_10_async_await_basics.rs`
> - **上一篇**：[9. 线程停放](./9-线程停放.md)
> - **下一篇**：[11. tokio 任务](./11-tokio任务.md)
> - **关键词**：`async`、`await`、`Future`、executor、惰性、`Poll`

---

## async/await 的本质

```
async fn foo() → fn foo() -> impl Future<Output = T>

Future 是惰性的：只有被 executor（运行时）驱动时才执行。

state machine（状态机）：
  每个 .await 点是一个暂停点
  编译器把 async 函数转成状态机枚举
```

---

## 核心抽象

```rust
// Future trait（简化）
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),   // 完成了
    Pending,    // 还没好，等通知
}
```

---

## 对比 OS 线程

| 特性 | OS 线程 | async/await |
|-----|--------|-----------|
| 内存（空闲时）| ~8MB 栈 | 仅状态机大小（KB）|
| 并发量 | 数百~千 | 数十万 |
| 阻塞调用 | OK | ❌ 会阻塞整个线程 |
| 适合场景 | CPU 密集 | IO 密集 |

---

## 使用 async（需要 runtime）

```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }

#[tokio::main]
async fn main() {
    let result = compute().await;
    println!("{result}");
}

async fn compute() -> i32 {
    // 模拟 IO 等待（不阻塞线程）
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    42
}
```

---

## 下一步

- 继续阅读：[11. tokio 任务](./11-tokio任务.md)
