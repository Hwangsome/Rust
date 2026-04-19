# 7. `anyhow`：应用层的万能错误类型

> - **所属章节**：第 11 章 · Error Handling
> - **Cargo package**：`chapter11`
> - **运行方式**：`cargo run -p chapter11`
> - **代码位置**：`chapters/chapter11/src/topic_07_anyhow_example.rs`
> - **上一篇**：[6. 分层结果 Option<Result>](./6-分层结果类型：OptionResult.md)
> - **下一篇**：[8. thiserror](./8-thiserror.md)
> - **关键词**：`anyhow`、`anyhow::Error`、`.context()`、应用层错误

---

## 这一节解决什么问题

在应用层代码里，你不关心错误的精确类型，只关心：
1. 操作成功了吗？
2. 失败了的话，错误信息是什么？

每次都定义 `enum AppError { ... }` 太繁琐。`anyhow` crate 提供了 `anyhow::Error`，可以装下任何实现了 `std::error::Error` 的错误，并支持添加上下文。

---

## 一分钟结论

- `anyhow::Error` 可以容纳任意错误类型（不需要 `From` 实现）
- `.context("...")` / `.with_context(|| ...)` 给错误添加上下文信息
- `bail!("...")` 相当于 `return Err(anyhow!("..."))`
- `ensure!(condition, "...")` 相当于 `if !condition { bail!(...) }`
- 适合应用层；库代码推荐用 `thiserror` + 精确类型

---

## 完整运行示例

```rust
use anyhow::{anyhow, bail, ensure, Context, Result};

fn read_and_parse(s: &str) -> Result<i32> {
    ensure!(!s.is_empty(), "输入不能为空");

    s.trim()
        .parse::<i32>()
        .with_context(|| format!("无法将 '{s}' 解析为整数"))
}

fn process(s: &str) -> Result<String> {
    let n = read_and_parse(s)
        .context("处理输入时失败")?;

    if n < 0 {
        bail!("不支持负数: {n}");
    }

    Ok(format!("处理结果: {}", n * 2))
}

fn main() {
    let test_cases = ["42", "", "abc", "-5", "100"];

    for input in test_cases {
        match process(input) {
            Ok(msg) => println!("✅ {:?} → {msg}", input),
            Err(e) => {
                println!("❌ {:?} → {e}", input);
                // 打印完整错误链
                for cause in e.chain().skip(1) {
                    println!("   原因: {cause}");
                }
            }
        }
    }
}
```

---

## anyhow vs thiserror 选型

| 场景 | 推荐 |
|-----|-----|
| 应用层（main.rs, CLI, API handler）| `anyhow` |
| 库代码（pub API）| `thiserror` + 自定义错误类型 |
| 调用方需要按错误类型分支处理 | `thiserror` |
| 只需要打印/记录错误 | `anyhow` |

---

## 下一步

- 继续阅读：[8. thiserror](./8-thiserror.md)
- 回到目录：[第 11 章：错误处理](./README.md)
