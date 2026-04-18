# 7. Barrier 同步

`std::sync::Barrier` 让一组线程在同一个点"**等齐了才继续**"。

## 对应代码

- [topic_07_synchronization_through_barriers.rs](../../chapters/chapter14/src/topic_07_synchronization_through_barriers.rs)

典型场景：map-reduce 的分阶段计算——等所有 worker 完成 phase-1，才一起进入 phase-2。
