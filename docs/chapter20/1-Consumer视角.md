# 1. Cargo Feature：消费者视角

```toml
[dependencies]
math = { path = "../math", default-features = false, features = ["stat"] }
```

建议：**关闭 default features**，明确列出真正需要的，减少依赖体积。

## 对应代码

- [topic_01_consumer_of_lib.rs](../../chapters/chapter20/src/topic_01_consumer_of_lib.rs)
