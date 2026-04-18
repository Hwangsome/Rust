# 10. async/await 基础

## 一分钟结论

- `async fn` / `async { ... }` 返回 `Future`，是"将来某时完成的值"
- Future 是**惰性**的，自己不会跑，必须由 **runtime / executor** 驱动
- `.await` 让当前 async 函数暂停，等 inner future 完成

## 对应代码

- [topic_10_async_await_basics.rs](../../chapters/chapter14/src/topic_10_async_await_basics.rs)

文件里手写了一个极简 `block_on` —— 用来说明 "executor 做了什么"。生产里用 tokio / async-std。
