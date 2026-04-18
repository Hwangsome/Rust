# 4. 隐式 bound

写 `&'a T` 时 Rust 自动补上 `T: 'a`。这避免了大量样板 bound。

## 对应代码

- [topic_04_background_for_variance_implied_bounds.rs](../../chapters/chapter16/src/topic_04_background_for_variance_implied_bounds.rs)
