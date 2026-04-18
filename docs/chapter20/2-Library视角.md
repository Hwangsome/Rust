# 2. Cargo Feature：库端视角

```toml
[features]
default     = ["basic_math"]
basic_math  = []
stat        = ["basic_math"]
advance     = ["dep:rust_math"]
```

- `feature_a = ["feature_b"]` 让 A 隐式打开 B
- `feature_x = ["dep:crate_name"]` 让 X 启用可选依赖

## 对应代码

- [topic_02_math.rs](../../chapters/chapter20/src/topic_02_math.rs)
