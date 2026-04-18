# 8. thiserror

- Cargo package: `chapter11`
- Run chapter: `cargo run -p chapter11`
- Chapter entry: `chapters/chapter11/src/main.rs`
- Reference module: `chapters/chapter11/src/topic_08_thiserror_example.rs`
- Chapter lab: `chapters/chapter11/src/lab.rs`

## 扩展演示输出（当前代码已升级）

`topic_08_thiserror_example.rs` 文件头给出 thiserror vs anyhow 的选型表：

| 维度 | thiserror | anyhow |
|-----|-----------|--------|
| 定位 | 精确 enum | 万能容器 |
| 调用方按 variant match | ✅ | ❌ |
| 层级 | library / 领域 | application / CLI |
| 代码量 | 稍多 | 极少 |

同时演示 `#[from]` 派生：让 `io::Error` 和 `ParseIntError` 通过 `?` 自动转换到 `AppError`。

## 定义

`thiserror` 是用来定义结构化错误类型的派生工具。它帮你少写样板代码，但错误类型本身依然是你自己定义的 enum 或 struct。

## 作用

- 保留明确的错误类型边界
- 自动实现 `Display`、`Error`、`From`
- 让调用者可以继续按错误变体分支处理

## 原理

通过 `#[derive(Error)]` 和 `#[from]` 这类属性，你可以很自然地把底层错误映射进自己的错误枚举，同时保持类型信息不丢失。

## 最小示例

```rust
#[derive(Debug, Error)]
enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] ParseIntError),
}
```

## 注意点

- thiserror 更适合库层、领域层和需要精细分支处理的代码
- 它是“帮你定义错误类型”，不是“帮你逃避错误设计”
- 是否保留具体变体，要看调用者是否真的需要

## 常见错误

- 以为 thiserror 和 anyhow 是替代关系，而不看边界
- 错误枚举设计得过粗或过细
- 明明不需要结构化区分，却过度设计错误层次

## 我的理解

thiserror 的价值在于“减少样板，但不削弱类型”。当错误本身是领域模型的一部分时，它很合适。

## 下一步

继续看 [错误处理里的方法链约束](./9-错误处理里的方法链约束.md)，把整章前面的内容和链式 API 重新连起来。
