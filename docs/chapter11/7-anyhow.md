# 7. anyhow

- Cargo package: `chapter11`
- Run chapter: `cargo run -p chapter11`
- Chapter entry: `chapters/chapter11/src/main.rs`
- Reference module: `chapters/chapter11/src/topic_07_anyhow_example.rs`
- Chapter lab: `chapters/chapter11/src/lab.rs`

## 定义

`anyhow` 是面向应用层的错误处理工具，核心思路是：统一错误类型，保留错误链，并方便补充上下文。

## 作用

- 减少自定义错误枚举样板代码
- 让应用层更容易向上传递错误
- 给错误增加上下文信息

## 原理

`anyhow::Result<T>` 本质上是 `Result<T, anyhow::Error>`。它适合“我更关心把错误顺着调用链传上去，并在高层集中处理”这种需求。

## 最小示例

```rust
let contents = fs::read_to_string(file_path)
    .with_context(|| format!("failed to read file: {}", file_path.display()))?;
```

## 注意点

- anyhow 更偏应用层，不太适合公开库 API
- 它适合快速统一多种错误来源
- `context` / `with_context` 是很高价值的功能

## 常见错误

- 在库层盲目使用 anyhow，导致调用者失去结构化错误信息
- 只图省事，不补任何上下文
- 以为用了 anyhow 就不用设计错误边界

## 我的理解

anyhow 像一个“应用层错误容器”。当我只想把错误带着上下文往上抛，它非常顺手。

## 下一步

继续看 [thiserror](./8-thiserror.md)，对比“统一错误容器”和“结构化错误定义”的差异。
