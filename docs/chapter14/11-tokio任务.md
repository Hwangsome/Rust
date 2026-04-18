# 11. tokio 任务

本章不引入 tokio 依赖，只给出心智模型：

- `#[tokio::main]` 宏在 main 入口创建 runtime
- `tokio::spawn(async { ... })` 把 future 交给 runtime，类似轻量线程
- 一个 runtime 可以跑几十万个 task，共享少量 worker OS 线程

## 对应代码

- [topic_11_tokio_tasks.rs](../../chapters/chapter14/src/topic_11_tokio_tasks.rs)

## 禁忌

- async 代码里**不要**调用阻塞 API（std::thread::sleep、std::fs::read 等）
- CPU 密集活从 async 里**挪出去**：`tokio::task::spawn_blocking(|| cpu_work())`
