# 12. CPU 密集任务

## 一分钟结论

- **CPU 密集** → 线程池（`std::thread` / `rayon`）
- **IO 密集** → async/await（tokio）
- 混合 → tokio 里用 `spawn_blocking`

## 对应代码

- [topic_12_computationally_expensive_functions.rs](../../chapters/chapter14/src/topic_12_computationally_expensive_functions.rs)

## 演示

同样的 `heavy_work(n)` 用单线程跑 2 次、再用 2 个线程并行跑 —— 后者耗时约一半。
