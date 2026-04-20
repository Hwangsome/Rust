# 2. Cargo Features：Library 视角

> - **所属章节**：第 21 章 · Structuring Projects
> - **代码位置**：`chapters/chapter21/src/topic_02_math.rs`
> - **上一篇**：[1. 消费者视角](./1-Consumer视角.md)
> - **下一篇**：本章最后一篇

---

## 在 library 里定义 features

```toml
[features]
# default = 默认启用的 features
default = ["basic_math"]

# 无依赖的 feature
basic_math = []

# 依赖其他 feature
statistics = ["basic_math"]

# 启用可选依赖
json_output = ["dep:serde", "dep:serde_json"]
advanced = ["dep:complex-math-lib"]

[dependencies]
serde = { version = "1", optional = true }
serde_json = { version = "1", optional = true }
```

---

## 在代码里使用

```rust
// src/lib.rs

#[cfg(feature = "basic_math")]
pub mod basic_math {
    pub fn add(a: f64, b: f64) -> f64 { a + b }
}

#[cfg(feature = "statistics")]
pub mod statistics {
    use crate::basic_math;

    pub fn mean(data: &[f64]) -> f64 {
        let sum: f64 = data.iter().sum();
        sum / data.len() as f64
    }
}
```

---

## 测试特定 feature

```bash
cargo test --features statistics
cargo test --all-features
cargo test --no-default-features --features basic_math
```

---

## 第 21 章完成

- 回到目录：[第 21 章：Structuring Projects](./README.md)
