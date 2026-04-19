# 1. Cargo Features：消费者视角

> - **所属章节**：第 20 章 · Structuring Projects
> - **Cargo package**：`chapter20`
> - **代码位置**：`chapters/chapter20/src/topic_01_consumer_of_lib.rs`

---

## 作为 library 的使用者

使用有 features 的库：

```toml
[dependencies]
# 只使用 JSON 功能（不引入 YAML 等）
serde = { version = "1", features = ["derive"] }

# 关闭默认 features，只启用需要的
tokio = { version = "1", default-features = false, features = ["io-util", "time"] }

# 多种方式获取当前启用的 features
cargo tree --features all
cargo metadata
```

---

## 在代码里使用条件编译

```rust
// 检查 feature 是否启用
#[cfg(feature = "my-feature")]
fn feature_specific() {
    println!("这个函数只在启用 my-feature 时存在");
}

// 在 doc 里标注
/// 需要 `feature = "json"` 才能使用
#[cfg(feature = "json")]
pub fn to_json(&self) -> String { todo!() }
```

---

## 下一步

- 继续阅读：[2. Library 视角](./2-Library视角.md)
