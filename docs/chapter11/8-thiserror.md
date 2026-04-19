# 8. `thiserror`：库代码的结构化错误

> - **所属章节**：第 11 章 · Error Handling
> - **Cargo package**：`chapter11`
> - **运行方式**：`cargo run -p chapter11`
> - **代码位置**：`chapters/chapter11/src/topic_08_thiserror_example.rs`
> - **上一篇**：[7. anyhow](./7-anyhow.md)
> - **下一篇**：[9. 错误处理里的方法链约束](./9-错误处理里的方法链约束.md)
> - **关键词**：`thiserror`、`#[error]`、`#[from]`、库错误、精确错误类型

---

## 这一节解决什么问题

库代码需要暴露精确的错误类型，让调用方能够**按 variant 分支处理**：

```rust
match db.connect() {
    Err(DbError::NotFound(id)) => { /* 重试 */ }
    Err(DbError::NetworkError(e)) => { /* 检查网络 */ }
    Err(DbError::AuthFailed) => { /* 检查凭证 */ }
    Ok(conn) => { /* 使用连接 */ }
}
```

手写这样的 enum 需要大量样板代码（`Display`、`Error`、`From`）。`thiserror` 用 derive 宏自动生成。

---

## 一分钟结论

- `#[derive(thiserror::Error, Debug)]` 在错误 enum 上
- `#[error("消息 {field}")]` 生成 `Display` 实现
- `#[from]` 在字段上自动生成 `From<SrcError>`（让 `?` 工作）
- 结果：精确的错误类型 + 最少样板代码
- 可以和 `anyhow` 混用：库用 `thiserror`，应用层用 `anyhow` 包装

---

## 完整运行示例

```rust
use thiserror::Error;
use std::{io, num::ParseIntError, path::PathBuf};

#[derive(Error, Debug)]
enum ConfigError {
    #[error("配置文件 '{path}' 不存在")]
    NotFound { path: PathBuf },

    #[error("配置文件读取失败")]
    Io(#[from] io::Error),

    #[error("配置值解析失败: {0}")]
    Parse(#[from] ParseIntError),

    #[error("配置值 {key} 必须在 [{min}, {max}] 范围内，实际为 {value}")]
    OutOfRange {
        key: String,
        value: i32,
        min: i32,
        max: i32,
    },
}

fn load_port(config_str: &str) -> Result<u16, ConfigError> {
    let port: i32 = config_str.trim().parse()?;  // → ConfigError::Parse

    if !(1..=65535).contains(&port) {
        return Err(ConfigError::OutOfRange {
            key: "port".to_string(),
            value: port,
            min: 1,
            max: 65535,
        });
    }

    Ok(port as u16)
}

fn main() {
    println!("=== thiserror 精确错误类型 ===");
    let test_cases = ["8080", "abc", "0", "99999", "443"];

    for input in test_cases {
        match load_port(input) {
            Ok(port) => println!("  ✅ {:?} → 端口 {port}", input),
            Err(ConfigError::Parse(e)) =>
                println!("  ❌ {:?} → 解析错误: {e}", input),
            Err(ConfigError::OutOfRange { value, min, max, .. }) =>
                println!("  ❌ {:?} → {value} 超出范围 [{min}, {max}]", input),
            Err(e) => println!("  ❌ {:?} → {e}", input),
        }
    }
}
```

---

## 两种错误框架对比

```
thiserror：
  #[derive(thiserror::Error)]
  enum MyError {
    #[error("...")]
    Variant(#[from] SomeError),
  }
  优点：精确类型，调用方可以 match
  用于：库代码，public API

anyhow：
  fn f() -> anyhow::Result<T> {
    do_something().context("step 1")?;
  }
  优点：无需定义类型，快速编写
  用于：应用层，不需要调用方 match 错误类型

两者可以组合：
  库暴露 thiserror 类型
  → 应用层用 anyhow 包装
  impl From<LibError> for anyhow::Error { ... }
  (自动通过 ?  实现)
```

---

## 下一步

- 继续阅读：[9. 错误处理里的方法链约束](./9-错误处理里的方法链约束.md)
- 回到目录：[第 11 章：错误处理](./README.md)
