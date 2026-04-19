# 11. tokio 任务

> - **所属章节**：第 14 章 · Concurrency
> - **代码位置**：`chapters/chapter14/src/topic_11_tokio_tasks.rs`
> - **上一篇**：[10. async/await 基础](./10-async基础.md)
> - **下一篇**：[12. CPU 密集任务](./12-CPU密集任务.md)

---

## tokio 的核心 API

```rust
// 启动一个异步任务（类似轻量线程）
let handle = tokio::spawn(async {
    // 异步工作
    42
});
let result = handle.await.unwrap();

// 并发等待多个 Future
let (r1, r2) = tokio::join!(future1(), future2());

// 让出控制权（让其他任务有机会运行）
tokio::task::yield_now().await;

// 设置超时
let result = tokio::time::timeout(
    std::time::Duration::from_secs(5),
    some_future()
).await;
```

---

## 重要规则

- **不要在 async 代码里调用阻塞 IO**（std::fs::read、std::thread::sleep...）
- 用 tokio 提供的异步版本：`tokio::fs`、`tokio::net`、`tokio::time::sleep`
- 必须阻塞时用 `tokio::task::spawn_blocking`

---

## 并发模式速查

```rust
// 并发等待所有完成
tokio::join!(task_a(), task_b(), task_c());

// 并发，任何一个完成就返回
tokio::select! {
    r = task_fast() => println!("fast: {r:?}"),
    r = task_slow() => println!("slow: {r:?}"),
}

// 收集多个任务结果
let tasks: Vec<_> = (0..10).map(|i| tokio::spawn(work(i))).collect();
let results: Vec<_> = futures::future::join_all(tasks).await;
```

---

## 下一步

- 继续阅读：[12. CPU 密集任务](./12-CPU密集任务.md)
